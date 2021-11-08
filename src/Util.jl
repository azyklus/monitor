export Optional, StringOrChar

const Optional{T} = Union{T, Missing}
const StringOrChar = Union{AbstractString, AbstractChar}
