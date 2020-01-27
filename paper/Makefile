.PHONY: all clean

all: paper.pdf

clean:
	rm -fv paper.{aux,bbl,bcf,blg,log,out,pdf,run.xml} paper-blx.bib

paper.pdf: paper.tex bibliography.bib
	pdflatex $< && biber paper && pdflatex $< && pdflatex $<
