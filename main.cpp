#include <iostream>

#include "kyber.hpp"
#include "sha3.hpp"
#include "base.hpp"

int main() {
	unsigned char* inArr = (unsigned char*)"Shake256 test vector";
	unsigned char* out = sha3::shake(256, 256, inArr, strlen((char*)inArr));
	std::cout << base16::encode(out, 256/8) << std::endl;

	return 0;
}
