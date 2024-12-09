use crate::*;

pub trait CalibrationDeltaInterface {
    type Object: CalibrationDeltaObject;

    fn get_object(&self, identifier: CalibrationIdentifier) -> &Self::Object;
    fn get_object_mut(&mut self, identifier: CalibrationIdentifier) -> &mut Self::Object;
    fn calibrate_delta(
        &mut self, 
        identifier: CalibrationIdentifier,
        delta_vote: DeltaVote,
        adjustment_factor: AdjustmentFactor,
    ) -> Result<(), CalibrationError>;
}

pub trait CalibrationDeltaObject {
    type Item: CalibrationItem;

    /// Retrieves an item by name.
    fn get_item(&self, name: &String) -> Option<Self::Item>;

    /// Sets an item in the calibration object.
    fn set_item(&mut self, name: &String, item: Self::Item);

    /// Retrieves all items in the calibration object.
    fn items(&self) -> Vec<Self::Item>;

    /// Performs calibration on the object.
    ///
    /// Adjusts percentages of the increase and decrease items based on the delta vote
    /// and adjustment factor. Ensures the total percentage remains valid.
    fn calibrate_delta(
        &mut self,
        delta_vote: DeltaVote,
        adjustment_factor: AdjustmentFactor,
    ) -> Result<(), CalibrationError> {
        let calibration_id: AccountId = CALIBRATION_ID.parse().expect("ERR_CALIBRATION_ID");
        if env::predecessor_account_id() != calibration_id {
            return Err(CalibrationError::Unauthorized);
        }

        if delta_vote.increase == delta_vote.decrease {
            return Err(CalibrationError::Other(
                "Increase and decrease items must be different".to_string(),
            ));
        }

        // Calculate the adjustment amount
        let adjustment = {
            let decrease_item = self
                .get_item(&delta_vote.decrease)
                .ok_or_else(|| CalibrationError::NotFound(delta_vote.decrease.clone()))?;

            match adjustment_factor {
                AdjustmentFactor::Percentage(percentage) => {
                    (decrease_item.get_percentage() * percentage as u128) / 100
                }
                AdjustmentFactor::Fixed(amount) => amount,
            }
        };

        // Ensure the decrease item has enough percentage to adjust
        {
            let decrease_item = self
                .get_item(&delta_vote.decrease)
                .ok_or_else(|| CalibrationError::NotFound(delta_vote.decrease.clone()))?;
            if decrease_item.get_percentage() < adjustment {
                return Err(CalibrationError::Underflow(delta_vote.decrease.clone()));
            }
        }

        // Adjust the increase item
        {
            let mut increase_item = self
                .get_item(&delta_vote.increase)
                .ok_or_else(|| CalibrationError::NotFound(delta_vote.increase.clone()))?;
            increase_item.set_percentage(
                increase_item
                    .get_percentage()
                    .checked_add(adjustment)
                    .ok_or_else(|| CalibrationError::Overflow(delta_vote.increase.clone()))?,
            );
            self.set_item(&delta_vote.increase, increase_item);
        }

        // Adjust the decrease item
        {
            let mut decrease_item = self
                .get_item(&delta_vote.decrease)
                .ok_or_else(|| CalibrationError::NotFound(delta_vote.decrease.clone()))?;
            decrease_item.set_percentage(
                decrease_item
                    .get_percentage()
                    .checked_sub(adjustment)
                    .ok_or_else(|| CalibrationError::Underflow(delta_vote.decrease.clone()))?,
            );
            self.set_item(&delta_vote.decrease, decrease_item);
        }

        // Validate total percentage
        if !self.validate_total_percentage() {
            return Err(CalibrationError::InvalidTotal);
        }

        Ok(())
    }

    /// Validates that the total percentage equals 100%.
    fn validate_total_percentage(&self) -> bool {
        let total: YoctoNumber = self.items().iter().map(|item| item.get_percentage()).sum();
        total == ONE_HUNDRED_YOCTO_NUMBER
    }
}


pub trait CalibrationItem {
    fn get_percentage(&self) -> YoctoPercentage;
    fn set_percentage(&mut self, percentage: YoctoPercentage);
}

#[derive(Debug)]
pub enum CalibrationError {
    DuplicateItem(String),
    Overflow(String),
    Underflow(String),
    InvalidTotal,
    NotFound(String),
    Other(String),
    Unauthorized,
}

impl std::fmt::Display for CalibrationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CalibrationError::DuplicateItem(name) => write!(f, "Item '{}' already exists", name),
            CalibrationError::Overflow(name) => write!(f, "Overflow occurred for item '{}'", name),
            CalibrationError::Underflow(name) => write!(f, "Underflow occurred for item '{}'", name),
            CalibrationError::InvalidTotal => write!(f, "Total percentage is invalid"),
            CalibrationError::NotFound(name) => write!(f, "Item '{}' not found", name),
            CalibrationError::Other(msg) => write!(f, "{}", msg),
            CalibrationError::Unauthorized => write!(f, "Unauthorized"),
        }
    }
}

impl std::error::Error for CalibrationError {}