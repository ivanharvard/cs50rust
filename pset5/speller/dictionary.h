// Declares a dictionary's functionality

#ifndef DICTIONARY_H
#define DICTIONARY_H

#include <stdbool.h>

// Maximum length for a word
// (e.g., pneumonoultramicroscopicsilicovolcanoconiosis)
#define LENGTH 45

// Prototypes
bool check(const char *word);
bool check_rs(const char *word);
unsigned int hash(const char *word);
unsigned int hash_rs(const char *word);
bool load(const char *dictionary);
bool load_rs(const char *dictionary);
unsigned int size(void);
unsigned int size_rs(void);
bool unload(void);
bool unload_rs(void);

#endif // DICTIONARY_H
