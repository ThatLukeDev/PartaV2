#ifndef tld_hash
#define tld_hash

#include <cmath>
#include <cstring>

namespace sha3 {
	template <typename T>
	T swap(T a) {
		T _out = a;
		for (unsigned int i = 0; i < sizeof(T); i++) {
			*((char*)(&_out) + i) = *((char*)(&a) + sizeof(T) - 1 - i);
		}
		return _out;
	}

	struct params {
		unsigned int rate;
		unsigned int w = 64;
	};

	unsigned char getBit(unsigned char b, unsigned char x) {
		return (b & (1 << (7-x))) >> (7-x);
	}
	unsigned char setBit(unsigned char b, unsigned char x, unsigned char v) {
		return (b & ~(1 << (7-x))) | (v << (7-x));
	}

	unsigned char rc(int t) {
		if (t % 255 == 0) {
			return 1;
		}

		unsigned char R = 0b10000000;

		for (int i = 1; i <= t % 255; i++) {
			unsigned char R8 = R & 0b00000001;
			R >>= 1;
			R = setBit(R, 0, getBit(R, 0) ^ R8);
			R = setBit(R, 4, getBit(R, 4) ^ R8);
			R = setBit(R, 5, getBit(R, 5) ^ R8);
			R = setBit(R, 6, getBit(R, 6) ^ R8);
		}

		return getBit(R, 0);
	}

	void Rnd(unsigned char* in, int ir, params _p) {
		unsigned int rT = int(std::log2(_p.w));

		unsigned char state[5][5][_p.w];
		for (int x = 0; x < 5; x++) {
			for (int y = 0; y < 5; y++) {
				for (int z = 0; z < _p.w; z++) {
					state[x][y][z] = in[((x*5)+y)*(_p.w)+z];
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
					d[x][z] = c[(x-1+5)%5][z] ^ c[(x+1)%5][(z-1+_p.w)%_p.w];
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
					in[((x*5)+y)*(_p.w)+z] = state[x][y][z];
				}
			}
		}
	}

	void keccak_f(unsigned char* in, params _p) {
		unsigned int rT = int(std::log2(_p.w));
		unsigned int nr = 12 + 2 * rT;

		unsigned char state[5][5][_p.w];
		for (int x = 0; x < 5; x++) {
			for (int y = 0; y < 5; y++) {
				for (int z = 0; z < _p.w; z++) {
					state[x][y][z] = getBit(in[((y*5)+x)*(_p.w/8)+z/8], z%8);
				}
			}
		}

		int rcount4 = 0;
		for (int ir = 12 + 2*rT - nr; ir < 12 + 2*rT; ir++) {
			Rnd((unsigned char*)state, ir, _p);
			rcount4++;
			if (rcount4 > 1)
				break;
		}

		int i = 0;
		for (int y = 0; y < 5; y++) {
			for (int x = 0; x < 5; x++) {
				for (int zD8 = 0; zD8 < _p.w/8; zD8++) {
					in[i] = state[x][y][zD8*8] << 7;
					for (int zM8 = 1; zM8 < 8; zM8++) {
						in[i] |= state[x][y][zD8*8+zM8] << (7-zM8);
					}

					i++;
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
