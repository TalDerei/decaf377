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

// c1
pub const TWO_ADICITY: u32 = 0x2f;

pub const QUADRATIC_NON_RESIDUE_TO_TRACE: Fq = Fq::from_montgomery_limbs([
    4340692304772210610,
    11102725085307959083,
    15540458298643990566,
    944526744080888988,
]);

pub const MULTIPLICATIVE_GENERATOR: Fq = Fq::from_montgomery_limbs([
    2984901390528151251,
    10561528701063790279,
    5476750214495080041,
    898978044469942640,
]);

pub const TWO_ADIC_ROOT_OF_UNITY: Fq = Fq::from_montgomery_limbs([
    12646347781564978760,
    6783048705277173164,
    268534165941069093,
    1121515446318641358,
]);

pub const FIELD_SIZE_POWER_OF_TWO: Fq = Fq::from_montgomery_limbs([
    2726216793283724667,
    14712177743343147295,
    12091039717619697043,
    81024008013859129,
]);
