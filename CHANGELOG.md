# Change Log

## Unreleased

## 0.3.1 -- 2020-10-23

- Speed up rectangle drawing
- Remove dependency on line_drawing
- Update sctk dev dependency to 0.12

## 0.3.0 -- 2020-05-27

- Raised MSRV to `1.41.0`.
- Upgraded dependency versions.

## 0.2.1 -- 2019-03-29

- Fix `get_width()` for texts that start and end with spaces

## 0.2.0 -- 2019-01-26

- **[Breaking]** Canvas is now endian aware and will draw to the buffer in the endianness of the `Endian` its created with

## 0.1.6 -- 2019-01-24

- Faster drawing of horizontal and verticle lines by precomputing line boundaries
- Only calculate alpha overlay when drawing colors without a non-max alpha value for performance

## 0.1.5 -- 2019-01-13

- Fix drawing of characters with negative bounding boxes
- Fix error in `get_width()` for text without any characters

## 0.1.4 -- 2018-11-10

- Remove rusttype version restriction

## 0.1.3 -- 2018-10-09

- Move from `quick-xml` to `xml-rs` dependency 

## 0.1.2 -- 2018-10-04

- Add basic/experimental support for fontconfig in `andrew::text::fontconfig`

## 0.1.1 -- 2018-09-17

- Manage dependencies to maintain rust 1.22 compatibility
- Update rusttype to 0.7.1

## 0.1.0 -- 2018-08-17

Initial version, including:

- canvas
- lines
- rectangles
- text
