use indoc::indoc;
use pinwheel::prelude::*;
use tangram_ui as ui;
use tangram_www_docs_inspect_common::{ThresholdMetrics, Tuning};
use tangram_www_layouts::{
	docs_layout::{DocsLayout, DocsPage, GettingStartedPage},
	document::Document,
};

#[derive(ComponentBuilder)]
pub struct Page {
	#[children]
	pub children: Vec<Node>,
}

impl Component for Page {
	fn into_node(self) -> Node {
		let threshold_metrics = vec![
			ThresholdMetrics {
				accuracy: 0.5696,
				f1_score: 0.5294,
				false_negatives: 17,
				false_positives: 2786,
				precision: 0.3614,
				recall: 0.9893,
				threshold: 0.05,
				true_negatives: 2133,
				true_positives: 1577,
			},
			ThresholdMetrics {
				accuracy: 0.7005,
				f1_score: 0.6121,
				false_negatives: 55,
				false_positives: 1895,
				precision: 0.4481,
				recall: 0.9654,
				threshold: 0.1,
				true_negatives: 3024,
				true_positives: 1539,
			},
			ThresholdMetrics {
				accuracy: 0.7595,
				f1_score: 0.6577,
				false_negatives: 89,
				false_positives: 1477,
				precision: 0.5046,
				recall: 0.9441,
				threshold: 0.15,
				true_negatives: 3442,
				true_positives: 1505,
			},
			ThresholdMetrics {
				accuracy: 0.8040,
				f1_score: 0.6920,
				false_negatives: 160,
				false_positives: 1116,
				precision: 0.5623,
				recall: 0.8996,
				threshold: 0.2,
				true_negatives: 3803,
				true_positives: 1434,
			},
			ThresholdMetrics {
				accuracy: 0.8238,
				f1_score: 0.7026,
				false_negatives: 239,
				false_positives: 908,
				precision: 0.5987,
				recall: 0.8500,
				threshold: 0.25,
				true_negatives: 4011,
				true_positives: 1355,
			},
			ThresholdMetrics {
				accuracy: 0.8430,
				f1_score: 0.7068,
				false_negatives: 362,
				false_positives: 660,
				precision: 0.6511,
				recall: 0.7728,
				threshold: 0.3,
				true_negatives: 4259,
				true_positives: 1232,
			},
			ThresholdMetrics {
				accuracy: 0.8486,
				f1_score: 0.6943,
				false_negatives: 474,
				false_positives: 512,
				precision: 0.6863,
				recall: 0.7026,
				threshold: 0.35,
				true_negatives: 4407,
				true_positives: 1120,
			},
			ThresholdMetrics {
				accuracy: 0.8542,
				f1_score: 0.6903,
				false_negatives: 536,
				false_positives: 413,
				precision: 0.7192,
				recall: 0.6637,
				threshold: 0.4,
				true_negatives: 4506,
				true_positives: 1058,
			},
			ThresholdMetrics {
				accuracy: 0.8558,
				f1_score: 0.6749,
				false_negatives: 619,
				false_positives: 320,
				precision: 0.7528,
				recall: 0.6116,
				threshold: 0.45,
				true_negatives: 4599,
				true_positives: 975,
			},
			ThresholdMetrics {
				accuracy: 0.8567,
				f1_score: 0.6591,
				false_negatives: 692,
				false_positives: 241,
				precision: 0.7891,
				recall: 0.5658,
				threshold: 0.5,
				true_negatives: 4678,
				true_positives: 902,
			},
			ThresholdMetrics {
				accuracy: 0.8567,
				f1_score: 0.6467,
				false_negatives: 740,
				false_positives: 193,
				precision: 0.8156,
				recall: 0.5357,
				threshold: 0.55,
				true_negatives: 4726,
				true_positives: 854,
			},
			ThresholdMetrics {
				accuracy: 0.8565,
				f1_score: 0.6311,
				false_negatives: 795,
				false_positives: 139,
				precision: 0.8518,
				recall: 0.5012,
				threshold: 0.6,
				true_negatives: 4780,
				true_positives: 799,
			},
			ThresholdMetrics {
				accuracy: 0.8486,
				f1_score: 0.5881,
				false_negatives: 890,
				false_positives: 96,
				precision: 0.8799,
				recall: 0.4416,
				threshold: 0.65,
				true_negatives: 4823,
				true_positives: 704,
			},
			ThresholdMetrics {
				accuracy: 0.8401,
				f1_score: 0.5383,
				false_negatives: 987,
				false_positives: 54,
				precision: 0.9183,
				recall: 0.3808,
				threshold: 0.7,
				true_negatives: 4865,
				true_positives: 607,
			},
			ThresholdMetrics {
				accuracy: 0.8289,
				f1_score: 0.4745,
				false_negatives: 1091,
				false_positives: 23,
				precision: 0.9562,
				recall: 0.3155,
				threshold: 0.75,
				true_negatives: 4896,
				true_positives: 503,
			},
			ThresholdMetrics {
				accuracy: 0.8166,
				f1_score: 0.4059,
				false_negatives: 1186,
				false_positives: 8,
				precision: 0.9807,
				recall: 0.2559,
				threshold: 0.8,
				true_negatives: 4911,
				true_positives: 408,
			},
			ThresholdMetrics {
				accuracy: 0.8120,
				f1_score: 0.3793,
				false_negatives: 1220,
				false_positives: 4,
				precision: 0.9894,
				recall: 0.2346,
				threshold: 0.85,
				true_negatives: 4915,
				true_positives: 374,
			},
			ThresholdMetrics {
				accuracy: 0.8051,
				f1_score: 0.3407,
				false_negatives: 1266,
				false_positives: 3,
				precision: 0.9909,
				recall: 0.2057,
				threshold: 0.9,
				true_negatives: 4916,
				true_positives: 328,
			},
			ThresholdMetrics {
				accuracy: 0.7557,
				f1_score: 0.0037,
				false_negatives: 1591,
				false_positives: 0,
				precision: 1.0,
				recall: 0.0018,
				threshold: 0.95,
				true_negatives: 4919,
				true_positives: 3,
			},
		];
		let p1 = ui::P::new()
			.child("Run ")
			.child(ui::InlineCode::new().child("tangram app"))
			.child(" and open your browser to ")
			.child(
				ui::Link::new()
					.href("http://localhost:8080".to_owned())
					.child("http://localhost:8080"),
			)
			.child(".");
		let p2 = ui::P::new()
			.child("Create a new repo and upload the ")
			.child(ui::InlineCode::new().child(".tangram"))
			.child(" file we just trained.");
		let callout = ui::Callout::new(ui::Level::Info)
			.title("Repo".to_owned())
			.child("A repo is where we can compare multiple versions of the same model.");
		let p3 = ui::P::new()
			.child("Click on 'Training Metrics' and have a look at the confusion matrix.");
		let p4 = ui::P::new().child("It looks like false negatives are a bit high. This means we are predicting people are healthy when they actually aren't. It would be better if the model had fewer false negatives, even if it means more false positives, because doctors can rule out heart disease with further testing. Let's make that change by going to the 'Tuning' page.");
		let p5 = ui::P::new()
			.child("Drag the tuning slider below and see how different thresholds affect precision and recall.");
		let p6 = ui::P::new()
			.child("When we lower the threhold, we predict that more people have heart disease which results in lower precision but higher recall.");
		let p7 = ui::P::new().child(
			"Once you've chosen a threshold, you can go back to your prediction code to use it.",
		);
		let prev_next_buttons = div()
			.class("docs-prev-next-buttons")
			.child(
				ui::Link::new()
					.href("predict".to_owned())
					.child("< Previous: Make a prediction."),
			)
			.child(
				ui::Link::new()
					.href("monitor".to_owned())
					.child("Next: Monitor your model in production. >"),
			);
		let content = ui::S1::new()
			.child(ui::H1::new().child("Inspect"))
			.child(
				ui::S2::new()
					.child(p1)
					.child(p2)
					.child(callout)
					.child(p3)
					.child(TrainingMetrics::new())
					.child(p4)
					.child(p5)
					.child(
						ui::Window::new()
							.child(Dehydrate::new("tuning", Tuning { threshold_metrics })),
					)
					.child(p6)
					.child(p7)
					.child(TuningCode::new()),
			)
			.child(prev_next_buttons);
		// let content = ui::Markdown::new(include_str("page.md")).nodes()
		let layout = DocsLayout::new(DocsPage::GettingStarted(GettingStartedPage::Inspect), None)
			.child(content);
		Document::new()
			.client("tangram_www_docs_inspect_client")
			.child(layout)
			.into_node()
	}
}

#[derive(ComponentBuilder)]
pub struct TrainingMetrics {
	#[children]
	pub children: Vec<Node>,
}

impl Component for TrainingMetrics {
	fn into_node(self) -> Node {
		ui::Window::new()
			.child(
				ui::S1::new()
					.child(ui::H1::new().child("Training Metrics"))
					.child(ui::ConfusionMatrix::new(
						"positive",
						Some(20),
						Some(19),
						Some(299),
						Some(400),
					)),
			)
			.into_node()
	}
}

#[derive(ComponentBuilder)]
pub struct TuningCode {
	#[children]
	pub children: Vec<Node>,
}

impl Component for TuningCode {
	fn into_node(self) -> Node {
		let code_for_language = ui::highlight_code_for_language(ui::CodeForLanguage {
			elixir: indoc!(
				r#"
					predict_options = %Tangram.PredictOptions{
						threshold: 0.5,
						compute_feature_contributions: false
					}
					output = Tangram.predict(model, input, predict_options)
				"#
			)
			.into(),
			go: indoc!(
				r#"
					predictOptions := tangram.PredictOptions{
						Threshold:                   0.5,
						ComputeFeatureContributions: false,
					}
					output := model.PredictOne(input, &predictOptions)
				"#
			)
			.into(),
			javascript: indoc!(
				r#"
					options = {
						threshold: 0.5,
						computeFeatureContributions: true
					}
					let output = model.predictSync(input, options)
				"#
			)
			.into(),
			python: indoc!(
				r#"
					predict_options = tangram.PredictOptions(
							threshold=0.5,
							compute_feature_contributions=True
					)
					output = model.predict(input, predict_options)
				"#
			)
			.into(),
			ruby: indoc!(
				r#"
					options = Tangram::PredictOptions.new(
						threshold: 0.5,
						compute_feature_contributions: true
					)
					output = model.predict(input, options: options)
				"#
			)
			.into(),
			rust: indoc!(
				r#"
					let options = tangram::PredictOptions {
						threshold: Some(0.5),
						compute_feature_contributions: Some(true),
					};
					let output = model.predict_one(input.clone(), Some(options.clone()));
				"#
			)
			.into(),
		});
		ui::Window::new()
			.child(ui::CodeSelect::new("predict-threshold", code_for_language))
			.into_node()
	}
}
