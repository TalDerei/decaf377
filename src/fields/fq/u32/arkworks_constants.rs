use super::Fq;

pub const MODULUS_LIMBS: [u64; 4] = [
    725501752471715841,
    6461107452199829505,
    6968279316240510977,
    1345280370688173398,
];

pub const MODULUS_MINUS_ONE_DIV_TWO_LIMBS: [u64; 4] = [
    9586122913090633728,
    12453925762954690560,
    3484139658120255488,
    672640185344086699,
];

pub const MODULUS_BIT_SIZE: u32 = 0xfd;

pub const TRACE_LIMBS: [u64; 4] = [
    17149038877957297187,
    11113960768935211860,
    14608890324369326440,
    9558,
];

pub const TRACE_MINUS_ONE_DIV_TWO_LIMBS: [u64; 4] = [
    8574519438978648593,
    5556980384467605930,
    7304445162184663220,
    4779,
];

pub const TWO_ADICITY: u32 = 0x2f;

pub const QUADRATIC_NON_RESIDUE_TO_TRACE: Fq = Fq::from_montgomery_limbs([
    959979442, 1010646183, 3969480491, 2585054627, 1500661798, 3618294908, 3318494364, 219914769,
]);

pub const MULTIPLICATIVE_GENERATOR: Fq = Fq::from_montgomery_limbs([
    4294966995, 694976511, 1879047879, 2459047525, 2458259049, 1275155277, 3112278384, 209309636,
]);

pub const TWO_ADIC_ROOT_OF_UNITY: Fq = Fq::from_montgomery_limbs([
    3661289032, 2944457293, 4231536044, 1579301595, 3002672421, 62522982, 1005564110, 261123163,
]);

pub const FIELD_SIZE_POWER_OF_TWO: Fq = Fq::from_montgomery_limbs([
    3093398907, 634746810, 2288015647, 3425445813, 3856434579, 2815164559, 4025600313, 18864871,
]);
