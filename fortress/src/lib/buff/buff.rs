#[derive(Copy, Clone, Deserialize)]
pub enum Buff {
    MoveSpeed,
    NumJumps,
    JumpStrength,

    SwordAttack,
    SwordKnockback,
    SwordSlashSpeed,

    CrossbowAttack,
    CrossbowFiringSpeed,
    ArrowSpeed,
    ArrowKnockback,
}
