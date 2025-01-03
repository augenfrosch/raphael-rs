use simulator::{action_mask, Action, ActionMask};

pub const PROGRESS_ACTIONS: ActionMask = action_mask!(
    Action::BasicSynthesis,
    Action::Veneration,
    Action::MuscleMemory,
    Action::CarefulSynthesis,
    Action::Groundwork,
    Action::DelicateSynthesis,
    Action::IntensiveSynthesis,
    Action::HeartAndSoul,
    Action::PrudentSynthesis
);

pub const QUALITY_ACTIONS: ActionMask = action_mask!(
    Action::BasicTouch,
    Action::Observe,
    Action::StandardTouch,
    Action::GreatStrides,
    Action::Innovation,
    Action::ByregotsBlessing,
    Action::PreciseTouch,
    Action::PrudentTouch,
    Action::Reflect,
    Action::PreparatoryTouch,
    Action::DelicateSynthesis,
    Action::AdvancedTouch,
    Action::TrainedFinesse,
    Action::RefinedTouch,
    Action::TrainedEye,
    Action::HeartAndSoul,
    Action::QuickInnovation
);

pub const DURABILITY_ACTIONS: ActionMask = action_mask!(
    Action::MasterMend,
    Action::TricksOfTheTrade,
    Action::WasteNot,
    Action::WasteNot2,
    Action::Manipulation,
    Action::ImmaculateMend,
    Action::TrainedPerfection
);
