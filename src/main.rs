use std::fs::File;
use std::io::{BufWriter, Write};
use rusqlite::Connection;

use anyhow;
use csv::ReaderBuilder;

fn csv_to_db() -> anyhow::Result<()> {
    let mut rdr = ReaderBuilder::new()
        .delimiter(b'|')
        .from_path(r"C:\Users\Melody\Downloads\wcvp\wcvp_names.csv")?;
    let mut conn = Connection::open(r"C:\Users\Melody\RustroverProjects\affinames\plampt.db")?;

    conn.execute(
        "CREATE TABLE  IF NOT EXISTS plants (
    plant_name_id INTEGER,
    ipni_id TEXT,
    taxon_rank TEXT,
    taxon_status TEXT,
    family TEXT,
    genus_hybrid TEXT,
    genus TEXT,
    species_hybrid TEXT,
    species TEXT,
    infraspecific_rank TEXT,
    infraspecies TEXT,
    parenthetical_author TEXT,
    primary_author TEXT,
    publication_author TEXT,
    place_of_publication TEXT,
    volume_and_page TEXT,
    first_published TEXT,
    nomenclatural_remarks TEXT,
    geographic_area TEXT,
    lifeform_description TEXT,
    climate_description TEXT,
    taxon_name TEXT,
    taxon_authors TEXT,
    accepted_plant_name_id INTEGER,
    basionym_plant_name_id INTEGER,
    replaced_synonym_author TEXT,
    homotypic_synonym TEXT,
    parent_plant_name_id INTEGER,
    powo_id TEXT,
    hybrid_formula TEXT,
    reviewed TEXT
);",
        (),
    )?;
    let tx = conn.transaction()?;

    {
        let mut instat = tx.prepare(
            "INSERT INTO plants (
        plant_name_id, ipni_id, taxon_rank, taxon_status, family,
        genus_hybrid, genus, species_hybrid, species, infraspecific_rank,
        infraspecies, parenthetical_author, primary_author, publication_author, place_of_publication,
        volume_and_page, first_published, nomenclatural_remarks, geographic_area, lifeform_description,
        climate_description, taxon_name, taxon_authors, accepted_plant_name_id, basionym_plant_name_id,
        replaced_synonym_author, homotypic_synonym, parent_plant_name_id, powo_id, hybrid_formula,
        reviewed
    ) VALUES (
        ?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10,
        ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20,
        ?21, ?22, ?23, ?24, ?25, ?26, ?27, ?28, ?29, ?30, ?31
    )",
        )?;
        let mut count = 0;
        for result in rdr.records() {
            // The iterator yields Result<StringRecord, Error>, so we check the
            // error here.
            let record = result?;
            // println!("{:?}", record);
            instat.execute(rusqlite::params_from_iter(record.iter().map(|s| {
                if s.is_empty() {
                    None // Rusqlite treats Option::None as SQL NULL
                } else {
                    Some(s)
                }
            })))?;
            count += 1;
            if count % 1000 == 0 {
                println!("Prepped {} records...", count);
            }

            // Assuming 'row' is a Vec<String> or similar iterable containing the 31 string values
        }
    }
    println!("committing transaction!");
    tx.commit()?;

    Ok(())
}


fn single() -> anyhow::Result<()> {
    let mut conn = Connection::open(r"C:\Users\Melody\RustroverProjects\affinames\plampt.db")?;

    let path = "families.md";
    let file = File::create(path)?;

    let mut writer = BufWriter::new(file);

    // 3. Write data using `write!`, `writeln!`, or `write_all`
    for fam in conn.prepare("select distinct family from plants order by family")?
        .query_map([], |row| {
            row.get::<usize, String>(0)
        })? {
        let fam = fam?;
        write!(writer, "{fam}, ")?;
    }
    // You can also write raw bytes

    // 4. Flush the buffer (optional, but recommended for certainty)
    // The buffer will also be flushed automatically when the `writer` goes out of scope,
    // but explicit flushing ensures any errors are handled properly.
    writer.flush()?;

    Ok(())
}

fn groups() -> anyhow::Result<()> {
    let mut conn = Connection::open(r"C:\Users\Melody\RustroverProjects\affinames\plampt.db")?;

    let path = "genus.md";
    let file = File::create(path)?;

    let mut writer = BufWriter::new(file);

    let mut current_group = ' ';

    // 3. Write data using `write!`, `writeln!`, or `write_all`
    for fam in conn.prepare("select distinct genus from plants order by genus")?
        .query_map([], |row| {
            row.get(0)
        })? {
        let fam: String = fam?;

        let first_char = fam.chars().next().unwrap_or_default();

        if first_char != current_group {
            current_group = first_char;
            writeln!(writer, "\n\n# {current_group}\n")?;
            println!("{current_group}");
        }


        write!(writer, "{fam}, ")?;
    }
    // You can also write raw bytes

    // 4. Flush the buffer (optional, but recommended for certainty)
    // The buffer will also be flushed automatically when the `writer` goes out of scope,
    // but explicit flushing ensures any errors are handled properly.
    writer.flush()?;

    Ok(())
}

fn main() -> anyhow::Result<()> {
    groups()?;
    Ok(())
}