# Progress

**Main components**

- Fetching
- Parsing
- Visited and to visit sets

## Fetching

- Http get call
- Save page to disk
- Notify parser threads

## Parsing

- Check that the page is non-duplicated: 
  - Compute simhash value of the page
  - Use a Bloom Filter to check is the page has already been downloaded
- Extract links from the page and add them to the to-visit set

## Visited and to visit sets

- Keep track of visited URLs via a Bloom Filter
- Start with an in-memory queue to visit URLs
- Design an on-disk structure to do the job