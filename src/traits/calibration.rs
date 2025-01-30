use crate::*;

pub trait CalibrationSingleInterface {
    fn calibrate_single(
        &mut self,
        id: ValueId,
        single_vote: SingleVote,
        adjustment_amount: YoctoNumber,
    );
}

pub trait CalibrationDeltaInterface {
    fn calibrate_delta(
        &mut self, 
        id: ValueId,
        delta_vote: DeltaVote,
        adjustment_amount: YoctoNumber,
    );
}
