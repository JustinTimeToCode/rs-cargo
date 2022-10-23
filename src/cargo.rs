use ascii::AsciiChar;
pub mod cargo {

    /*
     * Type used to represent an input character.  It is intended to
     * represent a Unicode code point (4 bytes max), so the C type
     * "char" is not used.  It is signed, so that we can represent
     * the out-of-band value EOF (-1) as a value of this type.
     */
    type CargoChar = i32;

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

    const CARGO_COLON: char = AsciiChar::Colon;
    const CARGO_LBRACE: char = AsciiChar::CurlyBraceOpen;
    const CARGO_RBRACE: char = AsciiChar::CurlyBraceClose;
    const CARGO_LBRACK: char = AsciiChar::BracketOpen;
    const CARGO_RBRACK: char = AsciiChar::BracketClose;
    const CARGO_QUOTE: char = AsciiChar::Quotation;
    const CARGO_BSLASH: char = AsciiChar::BackSlash;
    const CARGO_FSLASH: char = AsciiChar::Slash;
    const CARGO_COMMA: char = AsciiChar::Comma;
    const CARGO_PERIOD: char = AsciiChar::Dot;
    const CARGO_PLUS: char = AsciiChar::Plus;
    const CARGO_MINUS: char = AsciiChar::Minus;
    const CARGO_DIGIT0: char = AsciiChar::_0;
    const CARGO_B: char = AsciiChar::b;
    const CARGO_E: char = AsciiChar::e;
    const CARGO_F: char = AsciiChar::f;
    const CARGO_N: char = AsciiChar::n;
    const CARGO_R: char = AsciiChar::r;
    const CARGO_T: char = AsciiChar::t;
    const CARGO_U: char = AsciiChar::u;
    const CARGO_BS: char = AsciiChar::BackSpace;
    const CARGO_FF: char = AsciiChar::FF;
    const CARGO_LF: char = AsciiChar::LineFeed;
    const CARGO_CR: char = AsciiChar::CarriageReturn;
    const CARGO_HT: char = AsciiChar::Tab;
    const CARGO_SPACE: char = AsciiChar::Space;

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
    struct CargoString {
        capacity: usize,
        length: usize,
        content: &str,
    }

    impl CargoString {
        fn append_char(c: CargoChar) {}
    }

    /*
     * Structure used to hold a number.
     * The "text_value" field holds a printable/parseable representation of the number
     * as Unicode text, conforming to the Argo standard.
     * The "int_value" field holds the value of the number in integer format, if the
     * number can be exactly represented as such.
     * The "float_value" field holds the value of the number in floating-point format.
     * The "valid_text" field is nonzero if the "text_valid" field contains a valid
     * representation of the value.
     * The "valid_int" field is nonzero if the "int_value" field contains a valid
     * representation of the value.
     * The "valid_float" field is nonzero if the "float_value" field contains a valid
     * representation of the value.
     *
     * If multiple representations of the value of the number are present, they should
     * agree with each other.
     * It is up to an application to determine which representation is the appropriate
     * one to use, based on the semantics of the data being represented.
     */

    #[derive(Debug)]
    struct CargoNumber {
        string_value: CargoString,
        int_value: u64,
        float_value: f64,
        valid_string: char,
        valid_int: char,
        valid_float: char,
    }

    /*
     * Basic Cargo values, represented by the (unquoted) tokens
     * "true", "false", or "null" in Cargo code.
     */
    #[derive(Debug)]
    enum CargoBasic {
        CargoNull,
        CargoTrue,
        CargoFalse,
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
    struct CargoArray {
        element_list: CargoValue,
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
    struct CargoObject {
        member_list: CargoValue,
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
    struct CargoValue {
        cargo_type: CargoValueType,
        next: CargoValue,
        prev: CargoValue,
        name: CargoString,
    }

    impl CargoValue {}

    fn cargo_is_whitespace(c: char) -> bool {
        c == CARGO_SPACE || c == CARGO_LF || c == CARGO_CR || c == CARGO_HT
    }

    fn cargo_is_exponent(c: char) -> bool {
        c == CARGO_E || c == AsciiChar::E
    }

    fn cargo_is_digit(c: char) -> bool {
        c >= CARGO_DIGIT0 || c <= AsciiChar::_9
    }

    fn cargo_is_hex(c: char) -> bool {
        cargo_is_digit(c)
            || (c >= AsciiChar::A && c <= AsciiChar::F)
            || (c >= AsciiChar::a && c <= AsciiChar::f)
    }

    fn cargo_is_control(c: char) -> bool {
        c >= AsciiChar::Null && c < CARGO_SPACE
    }
}
