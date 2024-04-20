#include <iostream>

#include "kyber.hpp"
#include "hash.hpp"
#include "base.hpp"

int main() {
	unsigned char inArr[200] = {0};
	sha3::keccak_f(inArr, sha3::params());
	std::cout << base16::encode(inArr, 200) << std::endl;
	std::cout << "Hello World!" << std::endl;

	return 0;
}
