use spirv;

pub struct Type {
    pub id: spirv::Word,
    // None == Void
    pub ty: Option<Ty>,
}

pub enum Ty {
    Bool,
    Integer(Integer),
    Float(Float),
    Vector(Vector),
    Matrix(Matrix),
    Array(Array),
    Structure(Structure),
    Composite(Composite),
    Image(Image),
    Sampler(Sampler),
    SampledImage(SampledImage),
    ConcreteType(ConcreteType),
    AbstractType(AbstractType),
    OpaqueType(OpaqueType),
}

pub enum Scalar {
    Numerical(Numerical),
    Bool(Bool)
}

pub enum Numerical {
    Integer(Integer),
    Float(Float),
}

pub struct Bool {
    pub val: bool
}

pub struct Integer {
    pub bits: u64,
    pub width: u8,
    pub signdness: Signedness,
}

pub enum Signedness {
    Signed,
    Unsigned,
}

pub struct Float {
    pub bits: u64,
    pub width: u8,
}

pub struct Vector {
    pub ty: Scalar,
    pub size: usize,
}

pub struct Matrix {
    pub ty: Vector,
    pub size: usize,
}

pub struct Array {
    pub elements: Vec<Ty>,
}

pub struct Structure {
    pub members: Vec<Ty>,
}

pub enum Aggregate {
    Structure(Structure),
    Array(Array),
}

pub enum Composite {
    Aggregate(Aggregate),
    Matrix(Matrix),
    Vector(Vector),
}

pub struct Image {
    pub ty: Option<Scalar>,
    pub dim: spirv::Dim,
    pub depth: Option<bool>,
    pub arrayed: bool,
    pub multi_sampled: bool,
    pub sampled: Option<bool>,
    pub format: spirv::ImageFormat,
    pub access_qualifier: spirv::AccessQualifier,
}

pub struct Sampler;

pub struct SampledImage;

pub enum ConcreteType {

}

pub enum AbstractType {

}

pub enum OpaqueType {

}
