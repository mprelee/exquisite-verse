# Exquisite Verse

A simple application for creating and sharing poems with different levels of obfuscation.

## Features

- Create poems line by line
- View poems in three different formats:
  - Cleartext: Plain text
  - Semi-Obfuscated: All lines except the last one are base64 encoded
  - Fully-Obfuscated: All lines are base64 encoded
- Import and export poems
- Copy poems to clipboard

## Usage

### Native Application

To run the native application:

```bash
cargo run
```

### Web Application

To run the web application:

1. Open the `embed.html` file in your web browser, or
2. Run the server script and open http://localhost:8000/embed.html in your web browser:

```bash
./serve.sh
```

## Embedding in Other Websites

To embed the Exquisite Verse application in another website, you can:

1. Copy the contents of the `embed.html` file
2. Extract the CSS and JavaScript
3. Include them in your website

Alternatively, you can use an iframe:

```html
<iframe src="path/to/embed.html" width="800" height="600" frameborder="0"></iframe>
```

## License

This project is licensed under the MIT License - see the LICENSE file for details.