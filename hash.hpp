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
		unsigned int rT = std::sqrt(_p.w);

		unsigned char state[5][5][_p.w];
		for (int x = 0; x < 5; x++) {
			for (int y = 0; y < 5; y++) {
				for (int z = 0; z < _p.w; z++) {
					state[x][y][z] = getBit(in[x*y*(z/8)], z%8);
				}
			}
		}

		// theta
		{
			unsigned char c[5][_p.w];
			unsigned char d[5][_p.w];

			for (int x = 0; x < 5; x++) {
				for (int z = 0; z < _p.w; z++) {
					c[x][z] = state[x][0][z];
					c[x][z] ^= state[x][1][z];
					c[x][z] ^= state[x][2][z];
					c[x][z] ^= state[x][3][z];
					c[x][z] ^= state[x][4][z];
				}
			}

			for (int x = 0; x < 5; x++) {
				for (int z = 0; z < _p.w; z++) {
					d[x][z] = c[(x-1)%5][z] ^ c[(x+1)%5][(z-1)%_p.w];
				}
			}

			for (int x = 0; x < 5; x++) {
				for (int y = 0; y < 5; y++) {
					for (int z = 0; z < _p.w; z++) {
						state[x][y][z] ^= d[x][z];
					}
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
