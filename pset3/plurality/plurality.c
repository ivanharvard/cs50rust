#include <cs50.h>
#include <stdio.h>
#include <string.h>

// Max number of candidates
#define MAX 9

// Candidates have name and vote count
typedef struct
{
    string name;
    int votes;
} candidate;

// Array of candidates
candidate candidates[MAX];

// Number of candidates
int candidate_count;

// Function prototypes
bool vote(string name);
void print_winner(void);

void main_rs(void);
int main()
{
    main_rs();
    return 0;
}

bool vote_rs(string name, candidate *candidates, size_t candidate_count);
// Update vote totals given a new vote
bool vote(string name)
{
    return vote_rs(name, candidates, candidate_count);
}

void print_winner_rs(candidate *candidates, size_t candidate_count);
// Print the winner (or winners) of the election
void print_winner(void)
{
    return print_winner_rs(candidates, candidate_count);
}