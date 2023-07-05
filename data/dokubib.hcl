project "dsp-0804-project" {

  "howToCite" = "Dokumentationsbibliothek St. Moritz"

  "teaserText" = "Bibliothek St. Moritz Dokumentation is the local history archive of the community of St. Moritz, Switzerland."

  "datasets" = ["http://ns.dasch.swiss/repository#dsp-0804-dataset-000"]

  "description" = {
    "en" = "Bibliothek St. Moritz Dokumentation is the local history archive of the community of St. Moritz, Switzerland. It’s collection contains publications, manuscripts and audiovisual documents of the touristic development of St. Moritz"
  }

  "disciplines" = {
    "__type" = "URL"

    "type" = "Skos"

    "url" = "http://skos.um.es/unesco6/550301"

    "text" = "Local history"
  }

  "funders" = ["http://ns.dasch.swiss/repository#dsp-0804-organization-001"]

  "keywords" = {
    "en" = "Historic photograph"
  }

  "keywords" = {
    "en" = "Local history"
  }

  "keywords" = {
    "en" = "St. Moritz"
  }

  "keywords" = {
    "fr" = "Touristic development"
  }

  "name" = "Bilddatenbank Bibliothek St. Moritz"

  "shortcode" = "0804"

  "spatialCoverage" = {
    "__type" = "URL"

    "type" = "Geonames"

    "url" = "https://geonames.org/2658813"

    "text" = "Saint Moritz"
  }

  "startDate" = "1980-04-01"

  "temporalCoverage" = {
    "__type" = "URL"

    "type" = "Chronontology"

    "url" = "http://chronontology.dainst.org/period/mvhEZ4S2qWEa"

    "text" = "19th Century (1800 - 1899)"
  }

  "temporalCoverage" = {
    "__type" = "URL"

    "type" = "Chronontology"

    "url" = "http://chronontology.dainst.org/period/INtagfT8h7Fs"

    "text" = "20th and 21st Centuries"
  }

  "temporalCoverage" = {
    "__type" = "URL"

    "type" = "Chronontology"

    "url" = "http://chronontology.dainst.org/period/kqORhO4TGm4n"

    "text" = "20th Century (1900 - 1999)"
  }

  "url" = {
    "__type" = "URL"

    "type" = "URL"

    "url" = "https://data.dasch.swiss/dokubib/"

    "text" = "Project Website"
  }
}

datasets = {
  "__id" = "http://ns.dasch.swiss/repository#dsp-0804-dataset-000"

  "__type" = "Dataset"

  "__createdAt" = "1630601300976368000"

  "__createdBy" = "dsp-metadata-gui"

  "abstracts" = {
    "en" = "Bilddatenbank makes accessible the collection of historic photographs and other graphical representation of St. Moritz Dokumentationsbibliothek"
  }

  "accessConditions" = "restricted"

  "howToCite" = "Dokumentationsbibliothek St. Moritz"

  "languages" = {
    "de" = "Deutsch"

    "en" = "German"

    "fr" = "allemand"
  }

  "attributions" = {
    "__type" = "Attribution"

    "agent" = "http://ns.dasch.swiss/repository#dsp-0804-organization-000"

    "roles" = ["creator", "publisher"]
  }

  "status" = "Ongoing"

  "title" = "Dokumentationsbibliothek St. Moritz Bilddatenbank"

  "typeOfData" = ["Image", "Text"]
}

organizations = {
  "__id" = "http://ns.dasch.swiss/repository#dsp-0804-organization-001"

  "__type" = "Organization"

  "__createdAt" = "1630601301506212000"

  "__createdBy" = "dsp-metadata-gui"

  "address" = {
    "__type" = "Address"

    "street" = ""

    "postalCode" = "7500"

    "locality" = "St. Moritz"

    "country" = "Switzerland"
  }

  "name" = "Gemeinde St. Moritz"
}

organizations = {
  "__id" = "http://ns.dasch.swiss/repository#dsp-0804-organization-000"

  "__type" = "Organization"

  "__createdAt" = "1630601301561696000"

  "__createdBy" = "dsp-metadata-gui"

  "address" = {
    "__type" = "Address"

    "street" = "Plazza da Scoula 14"

    "postalCode" = "7500"

    "locality" = "St. Moritz"

    "country" = "Switzerland"
  }

  "email" = "doku@biblio-stmoritz.ch"

  "name" = "Dokumentationsbibliothek St. Moritz"

  "url" = {
    "__type" = "URL"

    "type" = "URL"

    "url" = "http://www.biblio-stmoritz.ch"

    "text" = "www.biblio-stmoritz.ch"
  }
}
