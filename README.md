<div align="center"><pre>
 _                ___                _   
| |              |__ \              | |  
| |_   ___ __  __   ) |  __ _  _ __ | |_ 
| __| / _ \\ \/ /  / /  / _` || '__|| __|
| |_ |  __/ )  (  / /_ | (_| || |   | |_ 
 \__| \___|/_/\_\|____| \__,_||_|    \__|
</pre></div>

<div align="center">

[![Status](https://img.shields.io/github/last-commit/follow39/text2art)](https://github.com/follow39/text2art)
[![Crates.io downloads](https://img.shields.io/crates/d/text2art)](https://crates.io/crates/text2art)
[![Crates.io downloads](https://img.shields.io/docsrs/text2art)](https://docs.rs/text2art/latest/text2art/#)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](/LICENSE)

</div>

---

<h1>Description</h1>
Simple library for translating text into ascii art. The library has several fonts at once, and the user can use their fonts from a file or string.<br> 

---

<h1>Install</h1>

~~~
text2art = "1.0.2"
~~~

---

<h1>How to use</h1>
Run

~~~
cargo run --example basic_usage
~~~

Code

~~~
use text2art::BasicFonts;
use text2art::Font;
use text2art::Printer;

fn main() {
    let font = match Font::from_basic(BasicFonts::Big) {
        Ok(font) => font,
        Err(_) => panic!("something wrong with font"),
    };
    let prntr = Printer::with_font(font);
    prntr.print_to_stdio("Welcome to tex2art! :)").ok();

    prntr.print_to_stdio("text for print_to_stdio").ok();
    prntr
        .print_to("text for print_to", &mut std::io::stdout())
        .ok();

    let rendered_text = prntr.render_text("text for render");
    match rendered_text {
        Ok(rendered_text) => println!("{}", rendered_text),
        Err(_) => println!("Something went wrong!"),
    }
}
~~~

Output
<pre>
                                                                                                                         __  
__          __       _                                   _              _                ___                _    _       \ \ 
\ \        / /      | |                                 | |            | |              |__ \              | |  | |    _  | |
 \ \  /\  / /   ___ | |  ___   ___   _ __ ___    ___    | |_   ___     | |_   ___ __  __   ) |  __ _  _ __ | |_ | |   (_) | |
  \ \/  \/ /   / _ \| | / __| / _ \ | '_ ` _ \  / _ \   | __| / _ \    | __| / _ \\ \/ /  / /  / _` || '__|| __|| |       | |
   \  /\  /   |  __/| || (__ | (_) || | | | | ||  __/   | |_ | (_) |   | |_ |  __/ )  (  / /_ | (_| || |   | |_ |_|    _  | |
    \/  \/     \___||_| \___| \___/ |_| |_| |_| \___|    \__| \___/     \__| \___|/_/\_\|____| \__,_||_|    \__|(_)   (_)/_/ 
 _                _        __                               _         _            _                        _        _  _        
| |              | |      / _|                             (_)       | |          | |                      | |      | |(_)       
| |_   ___ __  __| |_    | |_   ___   _ __     _ __   _ __  _  _ __  | |_         | |_   ___           ___ | |_   __| | _   ___  
| __| / _ \\ \/ /| __|   |  _| / _ \ | '__|   | '_ \ | '__|| || '_ \ | __|        | __| / _ \         / __|| __| / _` || | / _ \ 
| |_ |  __/ )  ( | |_    | |  | (_) || |      | |_) || |   | || | | || |_  ______ | |_ | (_) | ______ \__ \| |_ | (_| || || (_) |
 \__| \___|/_/\_\ \__|   |_|   \___/ |_|      | .__/ |_|   |_||_| |_| \__||______| \__| \___/ |______||___/ \__| \__,_||_| \___/ 
                                              | |                                                                                
                                              |_|                                                                                
 _                _        __                               _         _            _          
| |              | |      / _|                             (_)       | |          | |         
| |_   ___ __  __| |_    | |_   ___   _ __     _ __   _ __  _  _ __  | |_         | |_   ___  
| __| / _ \\ \/ /| __|   |  _| / _ \ | '__|   | '_ \ | '__|| || '_ \ | __|        | __| / _ \ 
| |_ |  __/ )  ( | |_    | |  | (_) || |      | |_) || |   | || | | || |_  ______ | |_ | (_) |
 \__| \___|/_/\_\ \__|   |_|   \___/ |_|      | .__/ |_|   |_||_| |_| \__||______| \__| \___/ 
                                              | |                                             
                                              |_|                                             
 _                _        __                                         _             
| |              | |      / _|                                       | |            
| |_   ___ __  __| |_    | |_   ___   _ __     _ __   ___  _ __    __| |  ___  _ __ 
| __| / _ \\ \/ /| __|   |  _| / _ \ | '__|   | '__| / _ \| '_ \  / _` | / _ \| '__|
| |_ |  __/ )  ( | |_    | |  | (_) || |      | |   |  __/| | | || (_| ||  __/| |   
 \__| \___|/_/\_\ \__|   |_|   \___/ |_|      |_|    \___||_| |_| \__,_| \___||_|   
</pre>

---

<h1>How to create font</h1>

You can use your font from str or file. You can use these functions

~~~
pub fn from_basic(font: basic_fonts::BasicFonts) -> Result<Font, FontError>
pub fn from_file<P: AsRef<std::path::Path>>(path: P) -> Result<Font, FontError>
~~~

Font rules:
1. Constant line width. All lines intro one grapheme should be with equal width
2. Font line must be in format. Font must contain 3 segments: grapheme, shift and data.

For example:
<pre>
'a':0:  __ _ \n / _` |\n| (_| |\n \__,_|\n
</pre>
Where:
<pre>
'a' - font grapheme (you can use any unicode grapheme here)
: - segmentation symbol
0 - shift
  __ _ \n / _` |\n| (_| |\n \__,_|\n - data
</pre>
3. Use "\n" for line segmentation
For example:
<pre>
 _   
| |  
| |_ 
| __|
| |_ 
 \__|
</pre>
Will be implemented as
<pre>
't':0: _   \n| |  \n| |_ \n| __|\n| |_ \n \__|\n
</pre>
4. Use shift if it needed. With shift you can move you grapheme up or down. All graphemes align on the zero line.
Examples from big font below.

Grapheme without shift
<pre>
't':0: _   \n| |  \n| |_ \n| __|\n| |_ \n \__|\n
 _    ___ 5
| |  
| |_  
| __|
| |_ 
 \__| ___ 0
|   |
1   5
Internal parameters:
Width - 5
Height - 6
Shift - 0
</pre>
Grapheme with negative shift
<pre>
'p':-2: _ __  \n| '_ \ \n| |_) |\n| .__/ \n| |    \n|_|    \n
 _ __   ___ 3
| '_ \ 
| |_) |
| .__/  ___ 0
| |    
|_|     __ -2
|     |
1     7
Internal parameters:
Width - 7
Height - 6
Shift - -2
</pre>
Grapheme with positive shift
<pre>
'"':3: _ _ \n( | )\n V V \n
 _ _  ___ 5
( | )
 V V  ___ 3
 
 
      ___ 0
|   |
1   5
Internal parameters:
Width - 5
Height - 3
Shift - 3
 </pre>
All examples in one structure
<pre>
 _           _ _ 
| |         ( | )
| |_  _ __   V V 
| __|| '_ \ 
| |_ | |_) |
 \__|| .__/       
     | |    
     |_|     
</pre>
5. You can leave comments. Use '#' for comments.

For example:
<pre>
# letters [a-z]
'a':0:  __ _ \n / _` |\n| (_| |\n \__,_|\n
'b':0: _     \n| |    \n| |__  \n| '_ \ \n| |_) |\n|_.__/ \n
</pre>

