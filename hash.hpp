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

	unsigned char rc(int t) {
		if (t % 255 == 0) {
			return 1;
		}

		unsigned char R = 0b10000000;

		for (int i = 1; i <= t % 255; i++) {
			R >>= 1;
			R = setBit(R, 0, getBit(R, 0) ^ getBit(R, 8));
			R = setBit(R, 4, getBit(R, 4) ^ getBit(R, 8));
			R = setBit(R, 5, getBit(R, 5) ^ getBit(R, 8));
			R = setBit(R, 6, getBit(R, 6) ^ getBit(R, 8));
		}

		return R;
	}

	void Rnd(unsigned char* in, int ir, params _p) {
		unsigned int rT = int(std::log2(_p.w));

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
					d[x][z] = c[(x-1+5)%5][z] ^ c[(x+1+5)%5][(z-1+_p.w)%_p.w];
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

		// rho
		{
			unsigned char a[5][5][_p.w] = {0};
			for (int z = 0; z < _p.w; z++) {
				a[0][0][z] = state[0][0][z];
			}

			int x = 1;
			int y = 0;

			for (int t = 0; t < 24; t++) {
				for (int z = 0; z < _p.w; z++) {
					a[x][y][z] = state[x][y][(z-(((t+1)*(t+2))/2)+_p.w)%_p.w];
				}
				int tmp = y;
				y = (2*x+3*y)%5;
				x = tmp;
			}

			memcpy(state, a, 5*5*_p.w);
		}

		// pi
		{
			unsigned char a[5][5][_p.w];

			for (int x = 0; x < 5; x++) {
				for (int y = 0; y < 5; y++) {
					for (int z = 0; z < _p.w; z++) {
						a[x][y][z] = state[(x+3*y)%5][x][z];
					}
				}
			}

			memcpy(state, a, 5*5*_p.w);
		}

		// chi
		{
			unsigned char a[5][5][_p.w];

			for (int x = 0; x < 5; x++) {
				for (int y = 0; y < 5; y++) {
					for (int z = 0; z < _p.w; z++) {
						a[x][y][z] = state[x][y][z] ^ ((a[(x+1)%5][y][z] ^ 1) & a[(x+2)%5][y][z]);
					}
				}
			}

			memcpy(state, a, 5*5*_p.w);
		}

		// iota
		{
			unsigned char RC[_p.w] = {0};

			for (int j = 0; j < rT; j++) {
				RC[int(std::pow(2, j))-1] = rc(j + 7*ir);
			}

			for (int z = 0; z < _p.w; z++) {
				state[0][0][z] ^= RC[z];
			}
		}

		for (int x = 0; x < 5; x++) {
			for (int y = 0; y < 5; y++) {
				for (int z = 0; z < _p.w; z++) {
					in[((x*5)+y)*(_p.w/8)+z/8] = setBit(in[((x*5)+y)*(_p.w/8)+z/8], z%8, state[x][y][z]);
				}
			}
		}
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
