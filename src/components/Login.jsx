import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import PropTypes from 'prop-types';
import { Storage, Text, State } from "../js";
import { useNavigate } from "react-router-dom";
import { useRecoilState} from "recoil";

export default function Login() {
  const [agent, setAgent] = useRecoilState(State.agentState);
  const [token, setToken] = useState("");
  const [name, setName] = useState("");
  const [faction, setFaction] = useState("");
  const [email, setEmail] = useState("");
  const [errorMessage, setErrorMessage] = useState("");
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
    get_my_agent();
  }

  async function register() {

  }

  async function get_my_agent() {
    await invoke("get_my_agent", { token: token }).then(response => {
      if (response.data) {
        setErrorMessage("");
        setToken(token);
        Storage.setToken(token);
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

        <div className="flex flex-row items-center justify-center lg:justify-start relative">
          <form
            className="w-full mr-2"
            onSubmit={(e) => {
              e.preventDefault();
              connect();
            }}
          >
            <input
              className="peer block text-black ml-2 p-1 rounded w-full"
              type="password"
              value={token}
              onChange={(e) => setToken(e.currentTarget.value)}
              placeholder="Enter your token..."
            />
            <label className="absolute left-3">Access Token</label>
            <button className="block ml-2 button mt-6 bg-gray-600" type="submit">Connect</button>
          </form>
          <form
            className="w-full ml-2 mr-4"
            onSubmit={(e) => {
              e.preventDefault();
              register();
            }}
          >
            <div className="relative mb-8">
              <input
                className="peer block text-black ml-2 p-1 rounded w-full"
                value={name}
                onChange={(e) => setName(e.currentTarget.value)}
                placeholder="Between 3-14 characters"
              />
              <label className="absolute left-3">Name</label>
            </div>
            <div className="relative mb-8">
              <input
                className="peer block text-black ml-2 p-1 rounded w-full"
                value={faction}
                onChange={(e) => setFaction(e.currentTarget.value)}
                placeholder="REPLACE TO DROPDOWN"
              />
              <label className="absolute left-3">Faction</label>
            </div>
            <div className="relative">
              <input
                className="peer block text-black ml-2 p-1 rounded w-full"
                value={email}
                onChange={(e) => setEmail(e.currentTarget.value)}
                placeholder="For reserved names"
              />
              <label className="absolute left-3">Email</label>
            </div>
            <button className="block ml-2 button mt-6 bg-gray-600" type="submit">Register</button>
          </form>
        </div>

        <p>{errorMessage}</p>
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
