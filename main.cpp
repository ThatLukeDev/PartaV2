#include <iostream>

#include "kyber.hpp"
#include "hash.hpp"
#include "base.hpp"

int main() {
	unsigned char* inArr = (unsigned char*)"Test vector sha3-256";
	unsigned char* out = sha3::digest(256, inArr, strlen((char*)inArr));
	std::cout << base16::encode(out, 256/8) << std::endl;

	return 0;
}
