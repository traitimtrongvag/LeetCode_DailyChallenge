/*
Time:
- Build: O(total chars in wordsContainer)
- Query: O(total chars in wordsQuery)

Space: O(total chars)

Idea:
Use array-based trie instead of pointers/unordered_map.

This saves a lot of memory because:
- no hashmap overhead
- no new/delete per node
- nodes stored compactly in vector

Each node stores:
- next 26 characters
- best index for this suffix
*/

class Solution {
public:
    struct TrieNode {
        int next[26];
        int bestIndex;

        TrieNode() {
            memset(next, -1, sizeof(next));
            bestIndex = -1;
        }
    };

    vector<TrieNode> trie;
    vector<string>* words;

    // choose better answer
    int betterIndex(int oldIndex, int newIndex) {
        if (oldIndex == -1) {
            return newIndex;
        }

        int oldLength = (*words)[oldIndex].size();
        int newLength = (*words)[newIndex].size();

        // smaller length is better
        if (newLength < oldLength) {
            return newIndex;
        }

        // if same length, smaller index is better
        if (newLength == oldLength && newIndex < oldIndex) {
            return newIndex;
        }

        return oldIndex;
    }

    void insertWord(string& word, int index) {
        int node = 0;

        // empty suffix answer
        trie[node].bestIndex =
            betterIndex(trie[node].bestIndex, index);

        // insert reversed word
        for (int i = word.size() - 1; i >= 0; i--) {
            int c = word[i] - 'a';

            // create node if needed
            if (trie[node].next[c] == -1) {
                trie[node].next[c] = trie.size();
                trie.push_back(TrieNode());
            }

            node = trie[node].next[c];

            // update best answer for this suffix
            trie[node].bestIndex =
                betterIndex(trie[node].bestIndex, index);
        }
    }

    int searchWord(string& query) {
        int node = 0;

        // default answer
        int answer = trie[node].bestIndex;

        // walk reversed query
        for (int i = query.size() - 1; i >= 0; i--) {
            int c = query[i] - 'a';

            // no more matching suffix
            if (trie[node].next[c] == -1) {
                break;
            }

            node = trie[node].next[c];
            answer = trie[node].bestIndex;
        }

        return answer;
    }

    vector<int> stringIndices(vector<string>& wordsContainer,
                              vector<string>& wordsQuery) {

        words = &wordsContainer;

        // reserve helps avoid reallocations
        trie.reserve(500000 + 5);

        // root node
        trie.push_back(TrieNode());

        // build trie
        for (int i = 0; i < wordsContainer.size(); i++) {
            insertWord(wordsContainer[i], i);
        }

        vector<int> answer;

        // answer queries
        for (string& query : wordsQuery) {
            answer.push_back(searchWord(query));
        }

        return answer;
    }
};