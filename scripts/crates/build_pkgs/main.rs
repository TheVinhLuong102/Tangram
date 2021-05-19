use clap::Clap;
use digest::Digest;
use duct::cmd;
use indoc::formatdoc;
use md5::Md5;
use sha1::Sha1;
use sha2::Sha256;
use std::path::{Path, PathBuf};
use tangram_error::Result;
use tangram_script_build::{Arch, Target, TargetFileNames};

#[derive(Clap)]
pub struct Args {
	#[clap(long, env)]
	version: String,
	#[clap(long, env, default_value = "https://pkgs.tangram.xyz")]
	url: String,
}

pub fn main() -> Result<()> {
	let args = Args::parse();

	let tangram_path = std::env::current_dir()?;
	let dist_path = tangram_path.join("dist");

	// Retrieve the keys from the password store.
	let alpine_public_key = cmd!("pass", "tangram/keys/alpine.public.rsa").run()?.stdout;
	let alpine_private_key = cmd!("pass", "tangram/keys/alpine.public.rsa").run()?.stdout;
	let deb_public_key = cmd!("pass", "tangram/keys/deb.public.gpg").run()?.stdout;
	let deb_private_key = cmd!("pass", "tangram/keys/deb.private.gpg").run()?.stdout;
	let rpm_public_key = cmd!("pass", "tangram/keys/rpm.public.gpg").run()?.stdout;
	let rpm_private_key = cmd!("pass", "tangram/keys/rpm.private.gpg").run()?.stdout;

	let alpine_public_key_path = tempfile::NamedTempFile::new()?.into_temp_path();
	std::fs::write(&alpine_public_key_path, alpine_public_key)?;
	let alpine_private_key_path = tempfile::NamedTempFile::new()?.into_temp_path();
	std::fs::write(&alpine_private_key_path, alpine_private_key)?;
	let deb_public_key_path = tempfile::NamedTempFile::new()?.into_temp_path();
	std::fs::write(&deb_public_key_path, deb_public_key)?;
	let deb_private_key_path = tempfile::NamedTempFile::new()?.into_temp_path();
	std::fs::write(&deb_private_key_path, deb_private_key)?;
	let rpm_public_key_path = tempfile::NamedTempFile::new()?.into_temp_path();
	std::fs::write(&rpm_public_key_path, rpm_public_key)?;
	let rpm_private_key_path = tempfile::NamedTempFile::new()?.into_temp_path();
	std::fs::write(&rpm_private_key_path, rpm_private_key)?;

	// Clean and create pkgs_path.
	let pkgs_path = dist_path.join("pkgs");
	let pkgs_path_exists = std::fs::metadata(&pkgs_path)
		.map(|m| m.is_dir())
		.unwrap_or(false);
	if pkgs_path_exists {
		std::fs::remove_dir_all(&pkgs_path)?;
	}
	std::fs::create_dir_all(&pkgs_path)?;

	alpine(
		&args,
		&dist_path,
		&pkgs_path,
		&alpine_public_key_path,
		&alpine_private_key_path,
	)?;
	deb(
		&args,
		&dist_path,
		&pkgs_path,
		&deb_public_key_path,
		&deb_private_key_path,
	)?;
	rpm(
		&args,
		&dist_path,
		&pkgs_path,
		&rpm_public_key_path,
		&rpm_private_key_path,
	)?;

	Ok(())
}

fn alpine(
	args: &Args,
	dist_path: &Path,
	pkgs_path: &Path,
	alpine_public_key_path: &Path,
	alpine_private_key_path: &Path,
) -> Result<()> {
	for arch in &[Arch::X8664, Arch::AArch64] {
		let repo_path = pkgs_path.join("stable").join("alpine");
		std::fs::create_dir_all(&repo_path)?;
		std::fs::copy(alpine_public_key_path, repo_path.join("tangram.rsa"))?;
		let apkbuild_path = repo_path.join("APKBUILD");
		let apkbuild = formatdoc!(
			r#"
				# Contributor: Tangram <root@tangram.xyz>
				# Maintainer: Tangram <root@tangram.xyz>
				pkgname=tangram
				pkgver={version}
				pkgrel=1
				pkgdesc="Tangram is an automated machine learning framework designed for programmers."
				url="https://www.tangram.xyz"
				arch={arch}
				license="MIT"
				source="tangram"
				\n
				check() {{
					:
				}}
				\n
				package() {{
					install -D -m 755 "$srcdir"/tangram "$pkgdir"/usr/bin/tangram
				}}
			"#,
			version = args.version,
			arch = match arch {
				Arch::X8664 => "x86_64",
				Arch::AArch64 => "aarch64",
			},
		);
		std::fs::write(&apkbuild_path, &apkbuild)?;
		let tangram_cli_dst_path = repo_path.join("tangram");
		let target = match arch {
			Arch::X8664 => Target::X8664UnknownLinuxGnu,
			Arch::AArch64 => Target::AArch64UnknownLinuxGnu,
		};
		let tangram_cli_path = dist_path
			.join(target.as_str())
			.join(TargetFileNames::for_target(target).tangram_cli_file_name);
		std::fs::copy(tangram_cli_path, &tangram_cli_dst_path)?;
		let script = r#"
		apk add build-base abuild
		echo "PACKAGER_PUBKEY=/tangram.public.rsa" >> /etc/abuild.conf
		echo "PACKAGER_PRIVKEY=/tangram.private.rsa" >> /etc/abuild.conf
		abuild -F checksum
		abuild -F -P $PWD
		rm -rf src pkg
	"#;
		cmd!(
			"docker",
			"run",
			"-i",
			"--rm",
			"-v",
			format!(
				"{}:{}",
				repo_path.canonicalize().unwrap().display(),
				"/tangram"
			),
			"-v",
			format!(
				"{}:{}",
				alpine_public_key_path.canonicalize().unwrap().display(),
				"/tangram.public.rsa"
			),
			"-v",
			format!(
				"{}:{}",
				alpine_private_key_path.canonicalize().unwrap().display(),
				"/tangram.private.rsa"
			),
			"-w",
			"/tangram",
			"alpine:3.13",
		)
		.stdin_bytes(script)
		.run()?;
		std::fs::remove_file(&apkbuild_path)?;
		std::fs::remove_file(&tangram_cli_dst_path)?;
	}
	Ok(())
}

fn deb(
	args: &Args,
	dist_path: &Path,
	pkgs_path: &Path,
	deb_public_key_path: &Path,
	deb_private_key_path: &Path,
) -> Result<()> {
	// Find all the .debs in args.debs.
	struct Deb {
		version: String,
		path: PathBuf,
	}
	let mut debs = Vec::new();
	for entry in std::fs::read_dir(dist_path.join("release"))? {
		let path = entry?.path();
		let is_deb = path
			.extension()
			.and_then(|e| e.to_str())
			.map(|e| e == "deb")
			.unwrap_or(false);
		if !is_deb {
			continue;
		}
		let stem = path.file_stem().unwrap().to_str().unwrap();
		let mut components = stem.split('_');
		let _ = components.next().unwrap();
		let version = components.next().unwrap().to_owned();
		let _ = components.next().unwrap().to_owned();
		debs.push(Deb { version, path });
	}
	let distributions = &["debian", "ubuntu"];
	let debian_versions = vec!["sid", "bullseye", "buster", "stretch"];
	let ubuntu_versions = vec!["groovy", "focal", "bionic"];
	let archs = vec!["amd64", "arm64"];
	for distribution in distributions {
		let repo_path = pkgs_path.join("stable").join(distribution);
		std::fs::create_dir_all(&repo_path)?;
		// Create the list files.
		let distribution_versions = match *distribution {
			"debian" => &debian_versions,
			"ubuntu" => &ubuntu_versions,
			_ => unreachable!(),
		};
		for distribution_version in distribution_versions {
			// Write the .list file.
			let list_path = repo_path.join(format!("{}.list", distribution_version));
			let list_file = formatdoc!(
				r#"
					# tangram packages for {1} {2}
					deb {0}/stable/{1} {2} main
				"#,
				args.url,
				distribution,
				distribution_version
			);
			std::fs::write(list_path, list_file)?;
			// Copy the public key.
			let public_key_path = repo_path.join(format!("{}.gpg", distribution_version));
			std::fs::copy(deb_public_key_path, public_key_path)?;
		}
		let pool_path = repo_path.join("pool");
		std::fs::create_dir(&pool_path)?;
		// Copy all the .debs into the pool.
		for deb in debs.iter() {
			std::fs::copy(&deb.path, pool_path.join(&deb.path.file_name().unwrap()))?;
		}
		let dists_path = repo_path.join("dists");
		std::fs::create_dir(&dists_path)?;
		for distribution_version in distribution_versions {
			let distribution_path = dists_path.join(distribution_version);
			std::fs::create_dir(&distribution_path)?;
			let mut md5_lines = Vec::new();
			let mut sha1_lines = Vec::new();
			let mut sha256_lines = Vec::new();
			for arch in &archs {
				let component_path = distribution_path.join("main");
				std::fs::create_dir(&component_path)?;
				let binary_arch_path = component_path.join(format!("binary-{}", arch));
				std::fs::create_dir(&binary_arch_path)?;
				let packages_path = binary_arch_path.join("Packages");
				let mut packages_file = String::new();
				for deb in debs.iter() {
					let deb_bytes = std::fs::read(&deb.path)?;
					let packages_entry = formatdoc!(
						r#"
							Package: tangram
							Version: {version}
							Architecture: {arch}
							Maintainer: Tangram <root@tangram.xyz>
							Filename: pool/tangram_{version}_{arch}.deb
							Size: {size}
							MD5sum: {md5}
							SHA1: {sha1}
							SHA256: {sha256}
							Homepage: https://www.tangram.xyz
							Description: Tangram is an automated machine learning framework designed for programmers.
						"#,
						version = deb.version,
						arch = arch,
						size = deb_bytes.len(),
						md5 = hex::encode(Md5::digest(&deb_bytes)),
						sha1 = hex::encode(Sha1::digest(&deb_bytes)),
						sha256 = hex::encode(Sha256::digest(&deb_bytes)),
					);
					packages_file.push_str(&packages_entry);
					packages_file.push('\n');
				}
				std::fs::write(&packages_path, &packages_file)?;
				let packages_file_len = packages_file.len();
				let packages_relative_path = packages_path
					.strip_prefix(&distribution_path)
					.unwrap()
					.display();
				let md5 = hex::encode(Md5::digest(packages_file.as_bytes()));
				let md5_line = format!(" {} {} {}", md5, packages_file_len, packages_relative_path);
				md5_lines.push(md5_line);
				let sha1 = hex::encode(Sha1::digest(packages_file.as_bytes()));
				let sha1_line =
					format!(" {} {} {}", sha1, packages_file_len, packages_relative_path);
				sha1_lines.push(sha1_line);
				let sha256 = hex::encode(Sha256::digest(packages_file.as_bytes()));
				let sha256_line = format!(
					" {} {} {}",
					sha256, packages_file_len, packages_relative_path,
				);
				sha256_lines.push(sha256_line);
			}
			// Write the Release file.
			let release_file_path = distribution_path.join("Release");
			let release_file = formatdoc!(
				r#"
					Codename: {}
					Architectures: amd64
					Components: main
					Date: {}
					Description: Packages from Tangram, Inc. (https://www.tangram.xyz)
					MD5Sum:
					{}
					SHA1:
					{}
					SHA256:
					{}
				"#,
				distribution_version,
				chrono::Utc::now().to_rfc2822(),
				md5_lines.join("\n"),
				sha1_lines.join("\n"),
				sha256_lines.join("\n"),
			);
			std::fs::write(&release_file_path, &release_file)?;
			// Write the Release.gpg file.
			cmd!(
				"sq",
				"sign",
				"--detached",
				"--signer-key",
				deb_private_key_path
			)
			.stdin_path(&release_file_path)
			.stdout_path(distribution_path.join("Release.gpg"))
			.read()?;
			// Write the InRelease file.
			cmd!(
				"sq",
				"sign",
				"--cleartext-signature",
				"--signer-key",
				deb_private_key_path
			)
			.stdin_path(&release_file_path)
			.stdout_path(distribution_path.join("InRelease"))
			.read()?;
		}
	}
	Ok(())
}

fn rpm(
	args: &Args,
	dist_path: &Path,
	pkgs_path: &Path,
	rpm_public_key_path: &Path,
	rpm_private_key_path: &Path,
) -> Result<()> {
	// Find all the .rpms in args.rpms.
	struct Rpm {
		target: String,
		path: PathBuf,
	}
	let mut rpms = Vec::new();
	for entry in std::fs::read_dir(dist_path.join("release"))? {
		let path = entry?.path();
		let is_rpm = path
			.extension()
			.and_then(|e| e.to_str())
			.map(|e| e == "rpm")
			.unwrap_or(false);
		if !is_rpm {
			continue;
		}
		let stem = path.file_stem().unwrap().to_str().unwrap();
		let mut components = stem.split('_');
		let _ = components.next().unwrap();
		let _ = components.next().unwrap().to_owned();
		let target = components.next().unwrap().to_owned();
		rpms.push(Rpm { target, path });
	}
	let targets = &["x86_64", "aarch64"];
	for (distribution, distribution_version) in &[
		("amazon-linux", Some("2")),
		("centos", Some("8")),
		("centos", Some("7")),
		("fedora", None),
		("rhel", Some("8")),
	] {
		let repo_path = pkgs_path
			.join("stable")
			.join(distribution)
			.join(distribution_version.unwrap_or(""));
		std::fs::create_dir_all(&repo_path)?;
		// Create the .repo file.
		let repo_file_path = repo_path.join("tangram.repo");
		let distribution_version_with_leading_slash = distribution_version
			.map(|v| format!("/{}", v))
			.unwrap_or_else(|| "".to_owned());
		let repo_file = formatdoc!(
			r#"
				[tangram]
				name=Tangram
				baseurl={0}/stable/{1}{2}/$basearch
				enabled=1
				type=rpm
				repo_gpgcheck=1
				gpgcheck=0
				gpgkey={0}/stable/{1}{2}/repo.gpg
			"#,
			args.url,
			distribution,
			distribution_version_with_leading_slash,
		);
		std::fs::write(repo_file_path, repo_file)?;
		// Copy the rpm public key.
		std::fs::copy(rpm_public_key_path, repo_path.join("repo.gpg"))?;
		#[allow(clippy::single_element_loop)]
		for target in targets {
			// Create the target dir.
			let repo_target_path = repo_path.join(target);
			std::fs::create_dir(&repo_target_path)?;
			// Copy the .rpm.
			for rpm in rpms.iter() {
				if rpm.target == *target {
					std::fs::copy(
						&rpm.path,
						repo_target_path.join(rpm.path.file_name().unwrap()),
					)?;
				}
			}
			// Run createrepo.
			cmd!("createrepo_c", &repo_target_path).run()?;
			// Write the signature.
			cmd!(
				"sq",
				"sign",
				"--detached",
				"--signer-key",
				rpm_private_key_path
			)
			.stdin_path(repo_target_path.join("repodata/repomd.xml"))
			.stdout_path(repo_target_path.join("repodata/repomd.xml.asc"))
			.read()?;
		}
	}
	Ok(())
}
