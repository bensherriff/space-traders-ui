import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import PropTypes from 'prop-types';
import { Storage, Text, State } from "../js";
import { useNavigate } from "react-router-dom";
import { useRecoilState} from "recoil";
import Input from "./Form/Input";
import { Button } from ".";
import Select from "./Form/Select";

export default function Login() {
  const [agent, setAgent] = useRecoilState(State.agentState);
  const [token, setToken] = useState("");
  const [tokenError, setTokenError] = useState("");
  const [name, setName] = useState("");
  const [nameError, setNameError] = useState("");
  const [factions, setFactions] = useState([]);
  const [faction, setFaction] = useState("");
  const [factionError, setFactionError] = useState("");
  const [email, setEmail] = useState("");
  const [emailError, setEmailError] = useState("");
  const [registerError, setRegisterError] = useState("");
  const [status, setStatus] = useState({});
  const navigate = useNavigate();

  useEffect(() => {
    var token = Storage.getToken();
    list_faction_strings();
    if (token) {
      setToken(token);
    }
    get_status();
  }, [])

  async function connect() {
    get_my_agent();
  }

  async function register() {
    await invoke("register", {faction: faction, symbol: name, email: email }).then(response => {
      if (response && response.data) {
        console.log(response);
        setToken(response.data.token);
        Storage.setToken(response.data.token);
        setAgent(response.data.agent);
        navigate("/fleet");
      } else if (response && response.error) {
        setRegisterError(response.error.message);
      }
    });
  }

  async function get_my_agent() {
    await invoke("get_my_agent", { token: token }).then(response => {
      if (response && response.data) {
        setTokenError("");
        setToken(token);
        Storage.setToken(token);
        setAgent(response.data);
        navigate("/fleet");
      } else if (response && response.error) {
        setTokenError(response.error.message);
      }
    });
  }

  async function list_faction_strings() {
    await invoke("list_faction_strings").then(response => {
      if (response && response.data) {
        setFactions(response.data);
        setFaction(response.data[0]);
      }
    });
  }

  async function get_status() {
    await invoke("get_status").then(response => {
      if (response && response.data) {
        setStatus(response.data);
      }
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

        <div className="flex flex-row items-center justify-center lg:justify-start relative">
          <form
            className="w-full mx-2"
            onSubmit={(e) => {
              e.preventDefault();
              connect();
            }}
          >
            <Input
              label="Token"
              type="password"
              placeholder="Enter your token..."
              errorMsg={tokenError}
              value={token}
              onChange={(e) => {
                setTokenError("");
                setToken(e.currentTarget.value);
              }}
            />
            <Button type="submit">Submit</Button>
          </form>
          <form
            className="w-full ml-2 mr-4"
            onSubmit={(e) => {
              e.preventDefault();
              register();
            }}
          >
            <Input
              label="Name"
              type="text"
              placeholder="Between 3-14 characters"
              errorMsg={nameError}
              value={name}
              onChange={(e) => {
                setNameError("");
                setName(e.currentTarget.value);
              }}
            />
            <Select
              label="Faction"
              errorMsg={factionError}
              list={factions}
              selected={faction}
              setSelected={setFaction}
            />
            <Input
              label="Email"
              type="text"
              placeholder="For reserved names"
              errorMsg={emailError}
              value={email}
              onChange={(e) => {
                setEmailError("");
                setEmail(e.currentTarget.value);
              }}
            />
            <Button type="submit">Register</Button>
            <p className="mt-2 text-sm text-red-600" id="text-error">
              {registerError}
            </p>
          </form>
        </div>
      </div>
      {status && status.announcements? (
        <div className="mt-4 mx-4 break-words">
          <i className="text-center flex">{status.description}</i>
          <ul>
            <li>Next Reset: {Text.date(status.serverResets.next)}</li>
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
