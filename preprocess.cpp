#include <regex.h>
#include <iostream>
#include <fstream>
#include <sstream>
#include <stdlib.h>

using namespace std;

int main(int argc, char** argv)
{
	if (argc != 2)
		return 1;

	string s = argv[1];
	cout << "Filename: " << s << endl;
	ifstream in(s.c_str());
	string t = s + ".md";
	ofstream out(t.c_str());

	string line;
	string tag("rcode");
	
	while (getline(in, line))
	{
		// check to see if the first part of the line matches our tag
		int result = line.compare(0, 5, tag);
	
		if (result == 0)
		{
			istringstream iss(line.c_str());
			string part, filename;

			iss >> filename; // consume rcode tag
			iss >> filename;

			iss >> part;
			int a = atoi(part.c_str()) - 1;
			
			iss >> part;
			int b = atoi(part.c_str()) - 1;

			cout << a << " " << b <<  " from " << filename << endl;

			ifstream code_in(filename.c_str());
			string code_line;

			if (a == -1 && b == -1) // read entire file
			{
				out << "```rust\n";
				while (getline(code_in, code_line))
				{
					out << 	code_line << "\n";
				}
				out << "```\n";
			}
			else
			{

				int i = 0;
				while (i < a)
				{
					getline(code_in, code_line); // eat lines
					i++;
				}
				out << "```rust\n";
				while (i <= b)
				{
					getline(code_in, code_line);
					out << code_line << "\n";
					i++;
				}
				out << "```\n";
			}
		}
		else
		{
			out << line << "\n";
		}
	}

	return 0;
}
