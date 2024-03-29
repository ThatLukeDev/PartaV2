#ifndef tld_kyber
#define tld_kyber

// SPEC: https://pq-crystals.org/kyber/data/kyber-specification-round3-20210804.pdf

#include <cstring>
#include <cmath>
#include <stdexcept>
#include <vector>
#include <random>

namespace kyber {
	const int n = 256;
	const int _n = 9;
	const int q = 3329;

	std::vector<unsigned char> BytesToBits(unsigned char* B, unsigned int k) {
		std::vector<unsigned char> _B(k*8);

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

	std::vector<unsigned char> BytesToBits(std::vector<unsigned char> B) {
		std::vector<unsigned char> _B(B.size() * 8);

		for (unsigned int i = 0; i < B.size(); i++) {
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

	double norm2(std::vector<double> w) {
		double _w = 0;

		for (double _wn : w) {
			_w += _wn * _wn;
		}
		_w = std::sqrt(_w);

		return _w;
	}

	double normInf(std::vector<double> w) {
		double _wMax = 0;
		bool change = false;

		for (double _wn : w) {
			double _wnC = reduce(_wn, q);

			if (_wnC > _wMax || change == false) {
				_wMax = _wnC;
				change = true;
			}
		}

		return _wMax;
	}

	int compress(int x, int d) {
		return reducePos(std::round((double(std::pow(2, d)) / q) * x), std::pow(2, d));
	}

	int decompress(int x, int d) {
		return std::round((q / double(std::pow(2, d))) * x);
	}
}

#endif
