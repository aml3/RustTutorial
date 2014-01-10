import sys
import os

from pygments import highlight
from pygments.lexers import RustLexer, JavaLexer
from pygments.formatters import HtmlFormatter

from io import StringIO

from ast import literal_eval

def process(name):
    # Format is: ptitle <title>
    TITLE_TAG = "ptitle"

    # Format is: phead <level> <heading>
    HEADING_TAG = "phead"

    # Format is: plink <url> <optional text>
    LINK_TAG = "plink"

    # Format is: scode <tabs?> <dict of arguments>
    CODE_TAG = "scode"

    # Format is: pnav <prev file> <next file>
    NAV_TAG = "pnav"

    # Format is: syntax
    SYNTAX_TAG = "synx" # wanted something that wouldn't be casually typed

    header = '''<!DOCTYPE html>
<html>
    <head>
        <meta charset="utf-8">
        <link href="css/style.css" rel="stylesheet" type="text/css">
        <link href="css/tabs.css" rel="stylesheet" type="text/css">
        <link href="css/code.css" rel="stylesheet" type="text/css">
        <script src="http://ajax.googleapis.com/ajax/libs/jquery/1.8.3/jquery.min.js"></script>
        <script src="js/tabs.js"></script>
    </head>
    <body>\n'''

    footer = '''    </body>
                </html>'''

    print("File is ", name)
    fin = open(name, 'r')
    fout = open("html/" + name.split('.')[0][9:] + ".html", 'w')
    print("Saving to html/", name.split('.')[0][9:], ".html")

    block_formatter = HtmlFormatter(linenos = True, cssclass = "src")
    snip_formatter = HtmlFormatter(linenos = False, cssclass = "src")
    lexers = {'rust': RustLexer(), 'java': JavaLexer()} 

    in_para = False
    tab_num = 1

    fout.write(header)
    # Note: You should lead with a heading.
    while True:
        line = fin.readline()
        if not line:
            break;
        tokens = line.replace('\n','').split(' ')
        #Cases for code snippets
        if tokens[0] == CODE_TAG:
            args = literal_eval(' '.join(tokens[2:]))
            keys = args.keys()
            if in_para:
                fout.write("</p>\n")
                in_para = False

            if 'loc' in keys:
                code = StringIO()
                f = open(args['loc'], 'r')
                if args['range']:
                    start = int(args['start']) if 'start' in keys else 0
                    stop = int(args['stop']) if 'stop' in keys else float("inf")
                else:
                    start = 0
                    stop = 0
                i = 1
                for l in f:
                    if start <= i and (not args['range'] or i <= stop):
                        code.write(l)
                    i += 1
                fout.write('<div class="notab">')
                fout.write(highlight(code.getvalue(), lexers[args['lang']], block_formatter) + '\n')
                fout.write('</div>')
                f.close()
                code.close()

            # No tabs and source files simultaneously for now. Would be easy to fix; I'm lazy.
            # Also doesn't support dynamic number of tabs. Again, easy fix.
            elif literal_eval(tokens[1]):
                rust = StringIO()
                java = StringIO()

                if fin.readline() != "rcode\n":
                    raise Exception("Bad code format!")
                l = fin.readline()
                while l != "edocr\n":
                    rust.write(l)
                    l = fin.readline()

                if fin.readline() != "jcode\n":
                    raise Exception("Bad code format!")
                l = fin.readline()
                while l != "edocj\n":
                    java.write(l)
                    l = fin.readline()

                fout.write('''  <ul class="tabs">
        <li><a href="#tab{0}-1">Rust</a></li>
        <li><a href="#tab{0}-2">Java</a></li>
    </ul>
    <div id="tab{0}-1" class="tabcode">
                    '''.format(tab_num))

                fout.write('<div class="tabbed">');
                fout.write(highlight(rust.getvalue(), lexers['rust'], block_formatter) + '\n')
                fout.write('</div>');
                fout.write('''\t\t\t\t\t</div>
                            
    <div id="tab{0}-2" class="tabcode">
                    '''.format(tab_num))

                fout.write('<div class="tabbed">');
                fout.write(highlight(java.getvalue(), lexers['java'], block_formatter) + '\n')
                fout.write('</div>');
                fout.write('''\t\t\t\t\t</div>''')                        
                tab_num += 1

            else:
                code = StringIO()
                l = fin.readline()
                if l == "codeb\n":
                    l = fin.readline()
                    while l != "bedoc\n":
                        code.write(l)
                        l = fin.readline()
                    fout.write('<div class="notab">');
                    fout.write(highlight(code.getvalue(), lexers[args['lang']], block_formatter) +'\n')
                    fout.write("</div>");

                elif l == "code\n":
                    in_para = True
                    l = fin.readline()
                    while l != "edoc\n":
                        code.write(l)
                        l = fin.readline()
                    high = highlight(code.getvalue(), lexers[args['lang']], snip_formatter)
                    fout.seek(fout.tell()-5)
                    fout.write('<span class="src"><code>' + high[22:-13].rstrip() + "</code></span>" + '\n')

                else:
                    raise Exception("Bad codeblock format!")
        #Case for syntax
        elif tokens[0] == SYNTAX_TAG:
            fout.write('<span class="src"><code>')
            code = StringIO()
            l = fin.readline()
            while l != "xnys\n":
                code.write(l)
                l = fin.readline()
            high = code.getvalue()
            code.close()
            parsed = StringIO()
            index = high.find("[")
            while index != -1:
                left = high[:index]
                right = high[index+1:]
                parsed.write(highlight(left, lexers[args['lang']], snip_formatter)[22:-13].rstrip())
                parsed.write('<span class="optional">')
                # handle ]
                high = right
                index = high.find("]")
                if index == -1:
                    raise Exception("Bad codeblock format!")
                left = high[:index]
                parsed.write(highlight(left, lexers[args['lang']], snip_formatter)[22:-13].rstrip())
                parsed.write('</span>')
                high = high[index+1:]
                index = high.find("[")
            fout.write(parsed.getvalue())
            fout.write('</code></span>')
            parsed.close()
                    
        #Case for page title 
        elif tokens[0] == TITLE_TAG:
            fout.write("<title>" + ' '.join(tokens[1:]) + "</title>\n")

        #Case for a heading    
        elif tokens[0] == HEADING_TAG:
            fout.write("<h" + tokens[1] + ">" + ' '.join(tokens[2:]) + "</h" + tokens[1] + ">\n")

        #Case for a link
        elif tokens[0] == LINK_TAG:
            if not in_para:
                fout.write("<p>\n")
            fout.write('<a href="' + tokens[1] + '">' + (tokens[1] if len(tokens) < 3 else ' '.join(tokens[2:])) + "</a>\n")
            if not in_para:
                fout.write("</p>\n")

        #Case for a paragraph break
        elif tokens == ['']:
            if in_para:
                fout.write("</p>\n")
                in_para = False

        #Case for the bottom-of-page navigation stuff
        elif tokens[0] == NAV_TAG:
            if in_para:
                fout.write("</p>\n")
                in_para = False
            fout.write('<hr/><a href="http://aml3.github.io/RustTutorial/html/' + tokens[1]+ '" style="float:left"> Previous </a>')
            #fout.write('<span style="width: 70% margin-left: auto margin-right: auto"><a href="http://aml3.github.io/RustTutorial/html/toc.html"> Table of Contents </a></span>')
            fout.write('<a href="http://aml3.github.io/RustTutorial/html/' + tokens[2]+ '" style="float:right"> Next </a>')

        #The default case
        else:
            if not in_para:
                fout.write("<p>\n")
                in_para = True
            fout.write(line)

    fin.close()
    fout.close()

if __name__ == "__main__":
    if len(sys.argv) != 2:
        print("exiting") 
        sys.exit()
    process(sys.argv[1])
