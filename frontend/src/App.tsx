import { useState } from 'react'
import './App.css'
import { InstantSearch, SearchBox, Hits, Highlight } from 'react-instantsearch';
import { instantMeiliSearch } from '@meilisearch/instant-meilisearch';
import {wikity} from 'wikity';

function App() {
  const [count, setCount] = useState(0)
  const searchClient = instantMeiliSearch(
    '127.0.0.1:7700',
    'aSampleMasterKey'
  );
  return (
    <div>
    <button>Connect Wallet</button>
  <InstantSearch
    indexName="wiki_pages"
    searchClient={searchClient.searchClient}
  >
    <SearchBox />
    <Hits hitComponent={Hit} />
  </InstantSearch>
  </div>
 );
}
const Hit = ({ hit }) => {
  return <div>
  <div>
  <h3>
    {hit.title}
    <button>Mint</button>
    <hr/>
    </h3>
    <div>
      {hit.body.split(" ").slice(0,50).join(" ")}
    </div>
    </div>
  </div>;
  <Highlight attribute="name" hit={hit.title.trim()} />
}

export default App
