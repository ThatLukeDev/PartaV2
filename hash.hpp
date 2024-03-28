#ifndef tld_hash
#define tld_hash

#include <cmath>
#include <cstring>

namespace sha3 {
	struct params {
		unsigned int rate;
		unsigned int w = 64;
	};

	unsigned char getBit(unsigned char b, unsigned char x) {
		return (b & (1 << x)) >> (x);
	}
	unsigned char setBit(unsigned char b, unsigned char x, unsigned char v) {
		return (b & ~(1 << x)) | (v << x);
	}

	unsigned char* keccak_f(unsigned char* in, params _p) {
		unsigned int t = std::sqrt(_p.w);

		unsigned char state[5][5][t];
		memcpy(state, in, t * 25);

		// theta : xor parity of each neighbouring column respectively
		{
			unsigned char c[5][t];
			unsigned char d[5][t];

			for (int x = 0; x < 5; x++) {
				for (int z = 0; z < _p.w; z++) {
					c[x][z] = setBit(c[x][z/8], z%8, getBit(state[x][0][z/8], z%8)
						^ getBit(state[x][1][z/8], z%8)
						^ getBit(state[x][2][z/8], z%8)
						^ getBit(state[x][3][z/8], z%8)
						^ getBit(state[x][4][z/8], z%8));
				}
			}
			for (int x = 0; x < 5; x++) {
				for (int z = 0; z < _p.w; z++) {
					d[x][z] = setBit(d[x][z/8], z%8, getBit(c[(x-1)%5][z/8], z%8)
						^ getBit(c[(x+1)%5][((z-1)%_p.w)/8],((z-1)%_p.w)%8));
				}
			}
			for (int x = 0; x < 5; x++) {
				for (int y = 0; y < 5; y++) {
					for (int z = 0; z < _p.w; z++) {
						state[x][y][z] = setBit(state[x][y][z/8], z%8, getBit(d[x][z/8], z%8)
							^ getBit(d[x][z/8], z%8));
					}
				}
			}
		}

		// rho : rotate each w by triangular numbers
		{
			unsigned char a[5][5][t];
			memcpy(a, state, t * 25);

			int x = 1;
			int y = 0;

			for (int i = 0; i < 24; i++) {
				for (int z = 0; z < _p.w; z++) {
					state[x][y][z/8] = setBit(state[x][y][z/8], z%8, getBit(a[x][y][(z-(((t+1)*(t+2))/2)%_p.w)/8], (z-(((t+1)*(t+2))/2)%_p.w)%8));
				int t = y;
				y = (2*x + 3*y) % 5;
				x = t;
				}
			}
		}

		return nullptr;
	}

	unsigned char* digest(unsigned char* _data, unsigned int _len, params _p) {
		unsigned int blockSize = _p.rate / 8;
		unsigned int blocks = std::ceil(double(_len + 2) / blockSize);
		unsigned int dataSize = blocks * blockSize;

		unsigned char* data = (unsigned char*)malloc(dataSize);
		for (unsigned int i = 0; i < dataSize; i++) data[i] = 0;
		memcpy(data, _data, _len);
		data[_len] = 1;
		data[dataSize - 1] = 1;

		return nullptr;
	}
}

#endif
