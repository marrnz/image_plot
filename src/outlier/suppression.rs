pub enum SuppressionStrategy {
    Thresholding,
    Clipping,
    LogarithmicScaling,
    Removing(RemoveConfig),
}

pub trait OutlierSuppression {
    fn suppress(&self, cells: &mut Vec<usize>);
}

struct ThresholdConfig;
struct ClipConfig;
struct LogarithmicScalingConfig;
pub struct RemoveConfig {
    pub threshold: usize,
}

impl OutlierSuppression for ThresholdConfig {
    fn suppress(&self, cells: &mut Vec<usize>) {
        todo!()
    }
}

impl OutlierSuppression for ClipConfig {
    fn suppress(&self, cells: &mut Vec<usize>) {
        todo!()
    }
}

impl OutlierSuppression for LogarithmicScalingConfig {
    fn suppress(&self, cells: &mut Vec<usize>) {
        todo!()
    }
}

impl OutlierSuppression for RemoveConfig {
    fn suppress(&self, cells: &mut Vec<usize>) {
        // elements can't be removed because it skews the grid
        // instead removing means zeroing the entry
        cells
            .iter_mut()
            .filter(|counter| **counter > self.threshold)
            .for_each(|counter| *counter = 0)
    }
}
