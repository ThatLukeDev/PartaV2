#ifndef tld_kyber
#define tld_kyber

#include <cstring>
#include <cmath>
#include <stdexcept>

namespace kyber {
	const int n = 256;
	const int _n = 9;
	const int q = 3329;

	unsigned char* BytesToBits(unsigned char* B, unsigned int k) {
		unsigned char* _B = (unsigned char*)malloc(k*8);

		for (unsigned int i = 0; i < k; i++) {
			_B[i*8+0] = (B[i] & 0b00000001) ? 1 : 0;
			_B[i*8+1] = (B[i] & 0b00000010) ? 1 : 0;
			_B[i*8+2] = (B[i] & 0b00000100) ? 1 : 0;
			_B[i*8+3] = (B[i] & 0b00001000) ? 1 : 0;
			_B[i*8+4] = (B[i] & 0b00010000) ? 1 : 0;
			_B[i*8+5] = (B[i] & 0b00100000) ? 1 : 0;
			_B[i*8+6] = (B[i] & 0b01000000) ? 1 : 0;
			_B[i*8+7] = (B[i] & 0b10000000) ? 1 : 0;
		}

		return _B;
	}

	int reduce(int r, int a) {
		if (a % 2 == 1) {
			a -= 1;
		}

		for (int _r = a/2; _r > -a/2; _r++) {
			if (_r == r % a) {
				return _r;
			}
		}

		throw std::invalid_argument("Given params to reduce do not have r % a in -a/2 < r < a/2");
	}

	int reducePos(int r, int a) {
		for (int _r = 0; _r < a; _r++) {
			if (_r == r % a) {
				return _r;
			}
		}

		throw std::invalid_argument("Given params to reduce do not have r % a in 0 < r < a");
	}
}

#endif
