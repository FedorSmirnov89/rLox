use crate::{domain::location::Location, parser::ErrorLocation};

#[test]
fn error_desc_long_input() {
    // Arrange
    let source_str = "this is the input we are processing;\n We assume an error here: X .\nWe also have a lot of other input after that.";
    let context = "test context";
    let location = Location {
        line: 2,
        column: 26,
        pos: 63,
    };
    let err_location = ErrorLocation::Position(location);

    // Act
    let desc = super::err_desc(source_str, &err_location, context);

    // Assert
    assert_eq!(desc.context, context);
    assert_eq!(
        desc.prefix,
        "nput we are processing;\n We assume an error here: "
    );
    assert_eq!(desc.highlighted, "X");
    assert_eq!(
        desc.suffix,
        " .\nWe also have a lot of other input after that."
    );
}

#[test]
fn error_desc_long_input_eof() {
    // Arrange
    let source_str = "this is the input we are processing;\n We assume an error here: X .\nWe also have a lot of other input after that.";
    let context = "test context";
    let err_location = ErrorLocation::EndOfInput;

    // Act
    let desc = super::err_desc(source_str, &err_location, context);

    // Assert
    assert_eq!(desc.context, context);
    assert_eq!(
        desc.prefix,
        ": X .\nWe also have a lot of other input after that"
    );
    assert_eq!(desc.highlighted, ".");
    assert_eq!(desc.suffix, "");
}

#[test]
fn error_desc_short_prefix() {
    // Arrange
    let source_str = "this is the input we are processing;\n We assume an error here: X .\nWe also have a lot of other input after that.";
    let context = "test context";
    let location = Location {
        line: 1,
        column: 5,
        pos: 2,
    };
    let err_location = ErrorLocation::Position(location);

    // Act
    let desc = super::err_desc(source_str, &err_location, context);

    // Assert
    assert_eq!(desc.context, context);
    assert_eq!(desc.prefix, "th");
    assert_eq!(desc.highlighted, "i");
    assert_eq!(
        desc.suffix,
        "s is the input we are processing;\n We assume an er"
    );
}

#[test]
fn error_desc_short_suffix() {
    // Arrange
    let source_str = "this is the input we are processing;\n We assume an error here: X .\nWe also have a lot of other input after that.";
    let context = "test context";
    let location = Location {
        line: 3,
        column: 1,
        pos: 101,
    };
    let err_location = ErrorLocation::Position(location);

    // Act
    let desc = super::err_desc(source_str, &err_location, context);

    // Assert
    assert_eq!(desc.context, context);
    assert_eq!(
        desc.prefix,
        "error here: X .\nWe also have a lot of other input "
    );
    assert_eq!(desc.highlighted, "a");
    assert_eq!(desc.suffix, "fter that.");
}
