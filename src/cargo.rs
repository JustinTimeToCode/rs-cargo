use ascii::AsciiChar;
use std::{
    error::Error,
    io::{self, BufReader, Stdin},
};

#[derive(Debug)]
enum CargoValueType {
    CargoNoType,
    CargoObjectType,
    CargoArrayType,
    CargoNumberType,
    CargoStringType,
    CargoBasicType,
}

/*
 * The following value is the maximum number of digits that will be printed
 * for a floating point value.
 */
const CARGO_PRECISION: i32 = 15;

/*
 * Constants that define the tokens used to represent the basic values
 * "true", "false", and "null", defined by the Cargo standard.
 */
const CARGO_TRUE_TOKEN: &str = "true";
const CARGO_FALSE_TOKEN: &str = "false";
const CARGO_NULL_TOKEN: &str = "null";

const CARGO_COLON: char = AsciiChar::Colon.as_char();
const CARGO_LBRACE: char = AsciiChar::CurlyBraceOpen.as_char();
const CARGO_RBRACE: char = AsciiChar::CurlyBraceClose.as_char();
const CARGO_LBRACK: char = AsciiChar::BracketOpen.as_char();
const CARGO_RBRACK: char = AsciiChar::BracketClose.as_char();
const CARGO_QUOTE: char = AsciiChar::Quotation.as_char();
const CARGO_BSLASH: char = AsciiChar::BackSlash.as_char();
const CARGO_FSLASH: char = AsciiChar::Slash.as_char();
const CARGO_COMMA: char = AsciiChar::Comma.as_char();
const CARGO_PERIOD: char = AsciiChar::Dot.as_char();
const CARGO_PLUS: char = AsciiChar::Plus.as_char();
const CARGO_MINUS: char = AsciiChar::Minus.as_char();
const CARGO_DIGIT0: char = AsciiChar::_0.as_char();
const CARGO_B: char = AsciiChar::b.as_char();
const CARGO_E: char = AsciiChar::e.as_char();
const CARGO_F: char = AsciiChar::f.as_char();
const CARGO_N: char = AsciiChar::n.as_char();
const CARGO_R: char = AsciiChar::r.as_char();
const CARGO_T: char = AsciiChar::t.as_char();
const CARGO_U: char = AsciiChar::u.as_char();
const CARGO_BS: char = AsciiChar::BackSpace.as_char();
const CARGO_FF: char = AsciiChar::FF.as_char();
const CARGO_LF: char = AsciiChar::LineFeed.as_char();
const CARGO_CR: char = AsciiChar::CarriageReturn.as_char();
const CARGO_HT: char = AsciiChar::Tab.as_char();
const CARGO_SPACE: char = AsciiChar::Space.as_char();

trait WriteCargo {
    fn write_cargo_cargo(&self, r: BufReader<Stdin>) -> Result<(), Box<dyn Error>>;
}

#[derive(Debug)]
pub enum CargoContent {
    Object(Box<CargoObject>),
    Array(Box<CargoArray>),
    String(CargoString),
    Number(CargoNumber),
    Basic(CargoBasic),
}

impl WriteCargo for CargoContent {
    fn write_cargo_cargo(&self, r: BufReader<Stdin>) -> Result<(), Box<dyn Error>> {
        match &self {
            CargoContent::Object(object) => object.write_cargo_object(r),
            CargoContent::Array(array) => array.write_cargo_array(r),
            CargoContent::String(string) => string.write_cargo_string(r),
            CargoContent::Number(number) => number.write_cargo_number(r),
            CargoContent::Basic(basic) => basic.write_cargo_basic(r),
            _ => Ok(()),
        }
    }
}

/*
 * Structure used to hold a string value.
 * The content field is maintained as an array of char, which is not null-terminated
 * and which might contain '\0' characters. This data is interpreted as Unicode text,
 * represented as an array of CargoChar values, each of which represents a single
 * Unicode code point. The length field gives the length in bytes of the data.
 * The capacity field records the actual size of the data area. This is included so
 * that the size can be dynamically increased while the string is being read.
 */
#[derive(Debug)]
pub struct CargoString {
    capacity: usize,
    length: usize,
    content: String,
}

impl CargoString {
    fn new(capacity: usize, length: usize, content: String) -> Self {
        Self {
            capacity,
            length,
            content,
        }
    }
    fn append_char(&mut self, c: char) {
        self.content.push(c);
        self.length += 1;
    }
    fn write_cargo_string(&self, r: BufReader<Stdin>) -> Result<(), Box<dyn Error>> {
        let cs: Self = Self {
            capacity: 0,
            length: 0,
            content: String::new(),
        };
        Ok(())
    }
}

fn read_cargo_string(r: BufReader<Stdin>) -> Result<CargoString, Box<dyn Error>> {
    Ok(CargoString {
        capacity: 10,
        length: 10,
        content: String::new(),
    })
}

/*
 * Structure used to hold a number.
 * The "text_value" field holds a printable/parseable representation of the number
 * as Unicode text, conforming to the Argo standard.
 * The "int_value" field holds the value of the number in integer format, if the
 * number can be exactly represented as such.
 * The "float_value" field holds the value of the number in floating-point format.
 *
 * If multiple representations of the value of the number are present, they should
 * agree with each other.
 * It is up to an application to determine which representation is the appropriate
 * one to use, based on the semantics of the data being represented.
 */

#[derive(Debug)]
pub struct CargoNumber {
    string_value: Option<CargoString>,
    int_value: Option<u64>,
    float_value: Option<f64>,
}

impl CargoNumber {
    fn write_cargo_number(&self, r: BufReader<Stdin>) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}

fn read_cargo_number(r: BufReader<Stdin>) -> Result<(), Box<dyn Error>> {
    Ok(())
}

/*
 * Basic Cargo values, represented by the (unquoted) tokens
 * "true", "false", or "null" in Cargo code.
 */
#[derive(Debug)]
pub enum CargoBasic {
    CargoNull,
    CargoTrue(bool),
    CargoFalse(bool),
}

impl CargoBasic {
    fn write_cargo_basic(&self, r: BufReader<Stdin>) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}
fn read_cargo_basic(r: BufReader<Stdin>) -> Result<CargoBasic, Box<dyn Error>> {
    Ok(CargoBasic::CargoTrue(true))
}

/*
 * An "array" has an ordered sequence of elements, each of which is just a value.
 * Here we represent the elements as a circular, doubly linked list, in the same
 * way as for the members of an object.  The "element_list" field in the CargoArray
 * structure serves as the sentinel at the head of the list.
 *
 * Note that elements of an array do not have any name, so the "name" field in each
 * of the elements will be NULL.  Arrays could be represented as actual arrays,
 * but we are not doing that here.
 */
#[derive(Debug)]
pub struct CargoArray {
    element_list: Option<CargoValue>,
}

impl CargoArray {
    fn write_cargo_array(&self, r: BufReader<Stdin>) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}
fn read_cargo_array(r: BufReader<Stdin>) -> Result<CargoArray, Box<dyn Error>> {
    Ok(CargoArray {
        element_list: Option::None,
    })
}

/*
 * An "object" has a list of members, each of which has a name and a value.
 * To store the members, we use a circular, doubly linked list, with the next and
 * previous pointers stored in the "next" and "prev" fields of the ARGO_VALUE structure
 * and the member name stored in the "name" field of the ARGO_VALUE structure.
 * The "member_list" field of the ARGO_OBJECT structure serves as the sentinel at
 * the head of the list.  This element does not represent one of the members;
 * rather, its "next" field points to the first member and its "prev" field points
 * to the last member.  An empty list of members is represented by the situation in
 * which both the "next" and "prev" fields point back to the sentinel object itself.
 *
 * Note that the collection of members of an object is supposed to be regarded as unordered,
 * which would permit it to be represented using a hash map or similar data structure,
 * which we are not doing here.
 */
#[derive(Debug)]
pub struct CargoObject {
    member_list: Option<CargoValue>,
}

impl CargoObject {
    fn write_cargo_object(&self, r: BufReader<Stdin>) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}
pub fn read_cargo_object(r: BufReader<Stdin>) -> Result<CargoObject, Box<dyn Error>> {
    Ok(CargoObject {
        member_list: Option::None,
    })
}

/*
 * The CargoValue structure is used to represent all kinds of Argo values.
 * The "type" field tells what type of value it represents.
 * It has "next" and "prev" fields so that it can be linked into "members"
 * or "elements" lists.  It has a "name" field which will hold the name in case
 * it is a member of an object.  The "content" field is the union of the structures
 * that represent the various Cargo types.  Depending on the value of the "type" field,
 * one of the "object", "array", or "string", "number", or "basic" variants of this union
 * will be valid.
 */
#[derive(Debug)]
pub struct CargoValue {
    cargo_type: CargoValueType,
    name: CargoString,
    content: CargoContent,
}

impl CargoValue {
    pub fn new(_type: CargoValueType, name: String) -> Self {
        Self {
            cargo_type: _type,
            name: CargoString {
                capacity: name.capacity(),
                length: name.len(),
                content: name,
            },
            content: match _type {
                CargoValueType::CargoObjectType | _ => {
                    CargoContent::Object(Box::new(CargoObject {
                        member_list: Option::None,
                    }))
                }
                CargoValueType::CargoArrayType => CargoContent::Array(Box::new(CargoArray {
                    element_list: Option::None,
                })),
            },
        }
    }
    fn write_cargo_object(&self, r: BufReader<Stdin>) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}

pub fn read_cargo_value() -> io::Result<CargoValue> {
    Ok(CargoValue::new(
        CargoValueType::CargoObjectType,
        "Sentinel".to_string(),
    ))
}

fn cargo_is_whitespace(c: char) -> bool {
    c == CARGO_SPACE || c == CARGO_LF || c == CARGO_CR || c == CARGO_HT
}

fn cargo_is_exponent(c: char) -> bool {
    c == CARGO_E || c == AsciiChar::E.as_char()
}

fn cargo_is_digit(c: char) -> bool {
    c >= CARGO_DIGIT0 || c <= AsciiChar::_9.as_char()
}

fn cargo_is_hex(c: char) -> bool {
    cargo_is_digit(c)
        || (c >= AsciiChar::A.as_char() && c <= AsciiChar::F.as_char())
        || (c >= AsciiChar::a.as_char() && c <= AsciiChar::f.as_char())
}

fn cargo_is_control(c: char) -> bool {
    c >= AsciiChar::Null.as_char() && c < CARGO_SPACE
}
