use algo_lib::misc::simulated_annealing::SimulatedAnnealing;

use crate::{
    dynamic_plot::DynamicPlot,
    hashcode_solver::{OneTest, Report},
    html_report::DynamicPlotId,
};

pub struct SaReport {
    score_plot: DynamicPlotId,
    delta_plot: DynamicPlotId,
    temp_plot: DynamicPlotId,
    accept_perc_plot: DynamicPlotId,
}

impl SaReport {
    pub fn new<'a>(report: &mut Report<'a>) -> Self {
        let score_plot = report.add_dynamic_plot(DynamicPlot::new(
            &"Score on each iteration of SA:",
            &"time (ms)",
            &"score",
        ));

        let delta_plot = report.add_dynamic_plot(DynamicPlot::new(
            &"Delta of scores checked by SA:",
            &"time (ms)",
            &"delta",
        ));

        let temp_plot = report.add_dynamic_plot(DynamicPlot::new(
            &"Temperature of SA:",
            &"time (ms)",
            &"temperature",
        ));

        let accept_perc_plot = report.add_dynamic_plot(DynamicPlot::new(
            &"Change accept % of SA:",
            &"time (ms)",
            &"accept %",
        ));

        Self {
            score_plot,
            delta_plot,
            temp_plot,
            accept_perc_plot,
        }
    }

    pub fn update(&mut self, test: &mut OneTest, sa: &SimulatedAnnealing) {
        let elapsed_ms = sa.elapsed_ms();
        self.score_plot.add_point(test, elapsed_ms, sa.last_score());
        self.temp_plot
            .add_point(test, elapsed_ms, sa.current_temperature());
        self.delta_plot.add_point(test, elapsed_ms, sa.last_delta());
        self.accept_perc_plot
            .add_point(test, elapsed_ms, sa.acceptance_percent());
    }
}
