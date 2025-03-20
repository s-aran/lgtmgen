# lgtmgen

LGTM image generator

## Usage

The program accepts the following command line arguments:

* **`-t`**, **`--text`**
  Specifies the text to be rendered.
  * Default value: `"LGTM"`
  * Example: `-t "Hello, World!"`

* **`-x`**
  Specifies the X coordinate for the text.
  * Default value: `"auto"`
    * If a specific numeric offset is desired, prefix with `+` or `-`.
  * Example: `-x "+50"`

* **`-y`**
  Specifies the Y coordinate for the text.
  * Default value: `"auto"`
    * If a specific numeric offset is desired, prefix with `+` or `-`.
  * Example: `-y "-20"`

* **`-c`**, **`--color`**
  Specifies the text color in HEX format.
  * Default value: `"#FFFFFF"`
  * Example: `-c "#FF0000"`

* **`-s`**, **`--size`**
  Specifies the font size for the text.
  * Default value: `200`
  * Example: `-s 150`

* **`-f`**, **`--font`**
  Specifies the path to the OTF/TTF font file.
  * Example: `-f ./fonts/Roboto-Regular.ttf`

* **`-i`**, **`--image`**
  Specifies the path to the background image file.
  * Example: `-i ./images/background.jpg`

* **`-o`, `--output`**  
  Specifies the output file path for the generated image.  
  * Default: `<input_filename_without_extension>_out.<input_extension>`
  * Example: `-i ./images/background_out.jpg`
