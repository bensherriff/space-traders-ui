import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import PropTypes from 'prop-types';
import { Storage, Text } from "../../js";
import { useNavigate } from "react-router-dom";
import { Button } from "..";

export default function Login( { setAgent } ) {
  const [token, setToken] = useState("");
  const [errorMessage, setErrorMessage] = useState("");
  const [loadDatabaseStatus, setLoadDatabaseStatus] = useState("Load Database Data");
  const [status, setStatus] = useState({});
  const navigate = useNavigate();

  useEffect(() => {
    var token = Storage.getToken();
    if (token) {
      setToken(token);
    }
    get_status();
  }, [])

  async function connect() {
    Storage.setSessionStorage('systems_db_loaded', false);
    get_my_agent();
  }

  async function load_database_data() {
    // setLoadDatabaseStatus("Loading Database");
    // await invoke("database_init", { token: token }).then(response => {
    //   if (response) {
    //     setLoadDatabaseStatus("Database Loaded");
    //     Storage.setSessionStorage('systems_db_loaded', true);
    //   } else {
    //     setLoadDatabaseStatus("Load Database Data");
    //   }
    // });
  }

  async function get_my_agent() {
    await invoke("get_my_agent", { token: token }).then(response => {
      if (response.data) {
        setErrorMessage("");
        setToken(token);
        Storage.setToken(token);
        Storage.setAgent(response.data);
        setAgent(response.data);
        navigate("/fleet");
      } else if (response.error) {
        setErrorMessage(response.error.message);
      }
    });
  }

  async function get_status() {
    await invoke("get_status").then(response => {
      setStatus(response.data);
    });
  }

  return (
    <div className="m-0 pt-10 flex flex-col">
      <div className="justify-center text-center">
        <h1 className="text-5xl mb-4 select-none text-sky-600 font-bold">Space Traders</h1>

        <div className="flex justify-center">
          <LoginLogo link={"https://spacetraders.io"} src={"/space-traders.svg"} alt={"Space Traders logo"}/>
          <LoginLogo link={"https://github.com/bensherriff/space-traders-ui"} src={"/github-mark-white.svg"} alt={"Github logo"}/>
        </div>

        <form
          className="mt-10 row flex flex-col justify-center items-center"
          onSubmit={(e) => {
            e.preventDefault();
            connect();
          }}
        >
          <textarea
            className="w-10/12 h-60"
            id="token-input"
            value={token}
            onChange={(e) => setToken(e.currentTarget.value)}
            placeholder="Enter your token..."
          />
          <button className="button w-11/12 mt-10 bg-gray-600" type="submit">Connect</button>
        </form>

        <p>{errorMessage}</p>
      </div>
      {status && status.announcements? (
        <div className="mt-4 mx-4 break-words">
          <i className="text-center flex">{status.description}</i>
          <ul>
            <li>Next Reset: {Text.date(status.serverResets.next)}</li>
            <li><Button disabled={loadDatabaseStatus != 'Load Database Data'} className="w-44" onClick={load_database_data}>{loadDatabaseStatus}</Button></li>
          </ul>
          <div className="mt-4 flex justify-between">
            <div className="mx-4 w-1/4">
              <h1 className="text-center text-2xl">Announcements</h1>
              {status.announcements.map((announcement, index) => (
                <div key={index} className="block bg-[#0f0f0f98] p-4 mb-2 rounded-lg">
                  <h2 key={`h_${index}`} className="text-sky-600 font-bold text-lg">{announcement.title}</h2>
                  <p key={`p_${index}`}>{announcement.body}</p>
                </div>
              ))}
            </div>
            <div className="mx-4 w-1/4">
              <h1 className="text-center text-2xl">Leaderboards</h1>
            </div>
            <div className="mx-4 w-1/4">
              <h1 className="text-center text-2xl">Stats</h1>
            </div>
            <div className="mx-4 w-1/4">
              <h1 className="text-center text-2xl">Links</h1>
              {status.links.map((link, index) => (
                <a key={index} className="block p-2 mb-2 bg-[#4b5563] hover:bg-[#2b3e58] shadow-md rounded-md select-none text-center" href={link.url}>{link.name}</a>
              ))}
            </div>
          </div>
        </div>
      ): <></>}
    </div>
  );
}

function LoginLogo({ link, src, alt }) {
  return (
    <a href={link} target="_blank">
      <img src={src} className="logo scale-150 mx-6" alt={alt}/>
    </a>
  )
}

Login.propTypes = {
  setAgent: PropTypes.func.isRequired
}
