/**
 * StringToINTERCAL.cs
 *
 * Converts a string to an INTERCAL script that will output said string.
 * usage: csc StringToINTERCAL.cs && ./StringToINTERCAL your string here
 *
 * @author Maxamilian Demian
 * @link https://www.maxodev.org
 * @link https://github.com/Maxoplata/StringToINTERCAL
 */
using System;

class StringToINTERCAL {
	private int politeCount = 0;

	private string politeLine(string line) {
		if (politeCount == 3) {
			politeCount = 0;

			return "PLEASE " + line + "\n";
		}

		politeCount++;

		return "DO " + line + "\n";
	}

	private string leadingZeros(string dec) {
		int count = dec.Length;

		if (count < 8) {
			dec = new string('0', (8 - count)) + dec;
		}

		return dec;
	}

	private string reverseString(string myString) {
		char[] charArray = myString.ToCharArray();

		Array.Reverse(charArray);

		return new string(charArray);
	}

	public string convertToINTERCAL(string myString) {
		// reset politeCount
		politeCount = 0;

		string ret = politeLine(",1 <- #" + myString.Length.ToString());

		int lastCharLoc = 256;
		for (int i = 0; i < myString.Length; i++) {
			int charLoc = Convert.ToInt32(reverseString(leadingZeros(Convert.ToString((int) myString[i], 2))), 2);

			int movePosition = 0;

			if (charLoc < lastCharLoc) {
				movePosition = (lastCharLoc - charLoc);
			} else if (charLoc > lastCharLoc) {
				movePosition = (256 - charLoc) + lastCharLoc;
			}

			lastCharLoc = lastCharLoc - movePosition;
			if (lastCharLoc < 1) {
				lastCharLoc = 256 + lastCharLoc;
			}

			ret = ret + politeLine(",1 SUB #" + (i + 1) + " <- #" + movePosition);
		}

		ret = ret + politeLine("READ OUT ,1");
		ret = ret + politeLine("GIVE UP");

		return ret;
	}
}

class App {
	static void Main(string[] args) {
		// if we have arguments passed to the script
		if (args.Length > 0) {
			string inputString = String.Join(" ", args);

			StringToINTERCAL myINTERCAL = new StringToINTERCAL();

			Console.WriteLine(myINTERCAL.convertToINTERCAL(inputString));
		}
	}
}
