# EasyJS UI

EasyJS provides a builtin UI library. Works directly with HTML,CSS, and JS. It is used instead of all the other stuff.

## Example
```javascript
import "ui/h"

// create the html object
html = ui.Html()
html.title = "Example UI Build"
html.styles.add_many([
        ui.Style(
            path="path_to_stylesheet.css"
        ),
        ui.Style(
            color=ui.Color.black,
            font=ui.Font.bold,
        ),
        ui.Style.from_string("
            color: #000000;
            font: font-bold;
        ")
    ]
)

// to add elements
h1 = ui.H1()
h1.append(ui.Paragraph(text="Hello World"))

html.body.add(h1)

// Run this at the end to compile into a (HTML, CSS, EasyJS) <-- website
html.compile()
```