#ifndef tld_hash
#define tld_hash

#include <cmath>
#include <cstring>

namespace sha3 {
	struct params {
		unsigned int rate;
		unsigned int w = 64;
	};

	unsigned char* keccak_f(unsigned char* state, params _p) {
		unsigned int t = std::sqrt(_p.w);

		unsigned char a[5][5][_p.w];
		memcpy(a, state, _p.w * 25);

		// theta
		{
			unsigned char c[5][_p.w];
			unsigned char d[5][_p.w];

			for (int x = 0; x < 5; x++) {
				for (int z = 0; z < _p.w; z++) {
					c[x][z] = a[x][0][z] ^ a[x][1][z] ^ a[x][2][z] ^ a[x][3][z] ^ a[x][4][z];
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
						a[x][y][z] ^= d[x][z];
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
