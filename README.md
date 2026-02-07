# plantnames

various organizations of scientific plant names when you need to name things in a planty way

# genus

these are the most "planty", different groupings are below:

- [name](genus_by_name.md)
- [name length](genus_by_length.md)
- [family](genus_by_family.md)
- [most common climate](genus_by_climate.md)
- [date first published](genus_by_first_published.md)
- [most common "lifeform" (what type of plant is it)](genus_by_lifeform.md)
- [count (how many species of this genus there are)](genus_by_count_grouped.md)
- [sorted by count, no grouping](genus_by_count.md)

# species

These are commonly just existing latin words that have little to do with plants, more distinguishing the member of the genus, but they're provided here.

There's also like 180k of em, so they're grouped by first two letters

[species](species_by_name.md)

# family

There are only 461 families, and all end in "aceae". No grouping is done.

[family](family.md)

# data source

Govaerts R (ed.). 2026. WCVP: World Checklist of Vascular Plants. Facilitated by the Royal Botanic Gardens, Kew. [WWW document] URL https://doi.org/10.34885/rvc3-4d77 [accessed 06 Jan 2026].

# code

rust code turns the csv into a sqlite database, which is then queried and then the results are written to markdown files. have fun :3
