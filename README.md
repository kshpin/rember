# rember

I forget things all the time. This is a tool to help me record short notes of ideas, thoughts, events, and other things that I can reference later on.

## Development plan

Need to first settle on a backend architecture.

Options up for comparison:
- cloudflare workers + D1
- self hosted + postgres

Criteria for choosing:
- service latency
  - i want to be able to type in a search box and have the database entries filter using fuzzy search on every keystroke
- long term cost
  - back of the napkin - with heavy usage (adding 480 notes per day for a year, each a sentence long), 35MB of new data per year
  - if self hosted, VPS cost, domain name, DNS
