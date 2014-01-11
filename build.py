import os
import sys
from process import process

#Just as a note, absolutely none of this (or the stuff in process.py) is robust to malformed input.
#Hooray for rush-coding!
def build(direc):
    os.chdir(direc)
    # Note: This currently rewrites the entire toc on each execution. I'm going to make 
    # this only do the parts in need of changing later, optimally.
    toc = open("html/toc.html", "w")
    header = '''<!DOCTYPE html>
<html>
    <head>
        <meta charset="utf-8">
        <link href="css/style.css" rel="stylesheet" type="text/css">
        <link href="css/tabs.css" rel="stylesheet" type="text/css">
        <link href="css/code.css" rel="stylesheet" type="text/css">
        <script src="http://ajax.googleapis.com/ajax/libs/jquery/1.8.3/jquery.min.js"></script>
        <script src="js/tabs.js"></script>
        <title> Table of Contents </title>
    </head>
    <body>
        <h1 style="text-align: center;"> Table of Contents </h1>
        <ol>'''

    footer = '''    </body>
                </html>'''

    levels = ['1','I','i','A','a']
    toc.write(header)
    for file in os.listdir("html/pre/"):
        if file.endswith(".page"):
            links = process("html/pre/"+file)
            path = file[:-5]+".html"
            text = links[0][0].replace('_', ' ')
            toc.write('<li> <a href="'+ path +'#' + links[0][0] + '">' + text + '</a> </li>\n')
            toc.write('<ol type="I">\n')
            level = 2
            for link in links[1:]:
                if link[1] > level:
                    toc.write('<ol type="'+levels[level % 5]+'">' +'\n')
                    level += 1
                elif link[1] < level:
                    toc.write('</ol>\n')
                    level -= 1
                text = link[0].replace('_', ' ')
                if text.rstrip()[-1] == ':':
                    text = text.rstrip()[:-1]
                toc.write('<li> <a href="'+ path +'#' + link[0] + '">' + text + '</a> </li>\n')
    toc.write('</ol>\n')
    toc.write('</ol>\n')
    toc.write(footer)
    toc.close()


if __name__ == "__main__":
    if len(sys.argv) != 2:
        print("exiting") 
        sys.exit()
    build(sys.argv[1])
