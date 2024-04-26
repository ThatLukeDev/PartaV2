#include <iostream>

#include "kyber.hpp"
#include "hash.hpp"
#include "base.hpp"

int main() {
	unsigned char* inArr = (unsigned char*)"Keccak-224 Test Hash";
	unsigned char* out = sha3::keccak(inArr, strlen((char*)inArr), 224, 448);
	std::cout << base16::encode(out, 224/8) << std::endl;

	return 0;
}
