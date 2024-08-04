#[derive(PartialEq, Copy, Clone)]
pub enum TurnState {
    AwaitingInput,
    PlayerTurn,
    EnemyTurn,
    GameOver,
    
}