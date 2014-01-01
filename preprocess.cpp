#include <regex.h>
#include <iostream>
#include <fstream>
#include <sstream>
#include <stdlib.h>

using namespace std;

int file_num(string path)
{
	int last = path.rfind('/');

	if (last == -1)
		return -1;

	string file = path.substr(last+1, path.length());
	return atoi(file.c_str());
}

void print_guide(int val, ofstream& out, bool end)
{
	if (end)
		out << "\n* * *\n";

	char buffer[32];
	if (val > 1)
	{
		if (val < 10)
			snprintf(buffer, 32, "[Previous](0%d.md)", val-1);
		else
			snprintf(buffer, 32, "[Previous](%d.md)", val-1);
		out << buffer;
	}

	if (val < 10)
		snprintf(buffer, 32, "[Next](0%d.md)", val+1);
	else
		snprintf(buffer, 32, "[Next](%d.md)", val+1);
	out << buffer;

	if (!end)
		out << "\n* * *\n";
}

int main(int argc, char** argv)
{
	if (argc != 2)
		return 1;

	string s = argv[1];
	ifstream in(s.c_str());
	cout << "Base Filename: " << s << endl;
	s = s.substr(0, s.length() - 3);
	int val = file_num(s);
	string t = s + ".md";
	ofstream out(t.c_str());
	
	print_guide(val, out, false);

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
	
	print_guide(val, out, true);

	return 0;
}
