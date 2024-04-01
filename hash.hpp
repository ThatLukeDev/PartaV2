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

	unsigned char getState(unsigned char* state, params _p, unsigned char x, unsigned char y, unsigned char z) {
		// state array is w(5y + x) + z
		unsigned char bit = _p.w*(5*y + x) + z;
		return getBit(state[bit/8], bit%8);
	}
	void setState(unsigned char* state, params _p, unsigned char x, unsigned char y, unsigned char z, unsigned char v) {
		// state array is w(5y + x) + z
		unsigned char bit = _p.w*(5*y + x) + z;
		state[bit/8] = setBit(state[bit/8], bit%8, v);
	}

	unsigned char getAdjState(unsigned char* state, params _p, unsigned char x, unsigned char y, unsigned char z) {
		// state array is x[3,4,0,1,2] y[3,4,0,1,2] z[0..63]
		return getState(state, _p, (x + 3) % 5, (y + 3) % 5, z);
	}
	void setAdjState(unsigned char* state, params _p, unsigned char x, unsigned char y, unsigned char z, unsigned char v) {
		// state array is x[3,4,0,1,2] y[3,4,0,1,2] z[0..63]
		setState(state, _p, (x + 3) % 5, (y + 3) % 5, z, v);
	}

	unsigned char* keccak_f(unsigned char* in, params _p) {
		unsigned int t = std::sqrt(_p.w);

		unsigned char* state;
		memcpy(state, in, t * 25);

		// theta : xor parity of each neighbouring column respectively
		{
			unsigned char c[5][t];
			unsigned char d[5][t];

			for (int x = 0; x < 5; x++) {
				for (int z = 0; z < _p.w; z++) {
					c[x][z] = setBit(c[x][z/8], z%8, getAdjState(state, _p, x, 0, z)
						^ getAdjState(state, _p, x, 1, z)
						^ getAdjState(state, _p, x, 2, z)
						^ getAdjState(state, _p, x, 3, z)
						^ getAdjState(state, _p, x, 4, z));
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
						setAdjState(state, _p, x, y, z, getBit(d[x][z/8], z%8)
							^ getBit(d[x][z/8], z%8));
					}
				}
			}
		}

		// rho : rotate each w by triangular numbers
		{
			unsigned char a[25*t];
			memcpy(a, state, t * 25);

			int x = 1;
			int y = 0;

			for (int i = 0; i < 24; i++) {
				for (int z = 0; z < _p.w; z++) {
					setAdjState(state, _p, x, y, z, getAdjState(a, _p, x, y, (z-(i+1)*(i+2)/2)) % _p.w);
				}

				int t = y;
				y = (2*x + 3*y) % 5;
				x = t;
			}
		}

		// pi : rearrange lanes
		{
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
