from urllib.request import urlopen, Request
from bs4 import BeautifulSoup
import re, csv, PyPDF2

characters = []

# Used to only scrap data chinese character and it's associated levels
urls = {
    'hsk1': 'https://mandarinbean.com/new-hsk-1-word-list/',
    'hsk2': 'https://mandarinbean.com/new-hsk-2-word-list/',
    'hsk3': 'https://mandarinbean.com/new-hsk-3-word-list/',
    'hsk4': 'https://mandarinbean.com/new-hsk-4-word-list/',
    'hsk5': 'https://mandarinbean.com/new-hsk-5-word-list/',
    'hsk6': 'https://mandarinbean.com/new-hsk-6-word-list/',
}

# headers needs to scrap the website
header= {'User-Agent': 'Mozilla/5.0 (X11; Linux x86_64) ' 
      'AppleWebKit/537.11 (KHTML, like Gecko) '
      'Chrome/23.0.1271.64 Safari/537.11',
      'Accept': 'text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8',
      'Accept-Charset': 'ISO-8859-1,utf-8;q=0.7,*;q=0.3',
      'Accept-Encoding': 'none',
      'Accept-Language': 'en-US,en;q=0.8',
      'Connection': 'keep-alive'}

class Hsk:
    def __init__(self, level, character, pinyin) -> None:
        self.level = level
        self.character = character
        self.pinyin = pinyin

    def remove_parenthese_character(self):
        trimmed = re.sub(r'\s*\（[^）]*\）', '', self.character)
        self.character = trimmed

    def __iter__(self):
        return iter([self.level, self.character, self.pinyin])

def process_hsk_url():
    print('Processing HSK website to scrap HSK1 - 6')
    for level, url in urls.items():
        print('Processing url: ' + url)
        # Used to store a list of chinese char for hsks
        req = Request(url=url, headers=header) 
        page = urlopen(req)

        html = page.read().decode('utf-8')
        # Use beautifulsoup to process the content of the html
        soup = BeautifulSoup(html, 'html.parser')
        # Get the element from the table
        td_elements = soup.find_all('td')
        # Loop through the td and only take chinese char and the id
        idx = 4
        while idx <= len(td_elements):
            character = td_elements[idx - 3].text.strip()
            pinyin = td_elements[idx - 2].text.strip()

            h = Hsk(level, character, pinyin)
            h.remove_parenthese_character()

            characters.append(h)

            idx += 4

# Process the hskupper pdf
def process_hsk_upper():
    print('Processing hsknewlevel.pdf')
    with open('../hsknewlevel.pdf', 'rb') as pdf_file:
        pdf_reader = PyPDF2.PdfReader(pdf_file)
        for page_num in range(len(pdf_reader.pages)):
            page = pdf_reader.pages[page_num]
            content = page.extract_text()
            # split the content by space
            contents = content.split('\n')
            for v in contents:
                # Check that we have at least a chinese chracter
                char = re.findall(r'[\u4e00-\u9fff]+', v)
                if len(char) > 0:
                    data = v.split(' ')
                    # It should at last contains the chinese character and the pinyin
                    if len(data) > 2:
                        h = Hsk('hsk7-9', data[0], data[1])
                        characters.append(h)

# Run
process_hsk_url()
process_hsk_upper()

# Export the data into csv
with open('../hsk.csv', 'w') as stream:
    writer = csv.writer(stream, delimiter=';')
    writer.writerow(['level', 'character', 'pinyin'])
    writer.writerows(characters)
