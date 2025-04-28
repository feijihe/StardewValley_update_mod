import { useState, useMemo } from "react";
import { Button, Table, TableColumnsType, Spin } from "antd";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";
import { open } from '@tauri-apps/plugin-dialog';

interface Mod {
  id: string; // 唯一标识符
  name: string; // 名称
  version: string; // 版本 
  latest_version?: string; // 最新版本
  // description: string; // 描述
}

function App() {
  const [modsDirectory, setModsDirectory] = useState<string>(''); // 存储目录
  const [mods, setMods] = useState<Mod[]>([]); // 存储mods lis

  async function handleSelectDirectory() {
    const selected = await open({
      directory: true,
      multiple: false,
      title: '选择存储目录'
    });

    if (typeof selected === 'string') {
      console.log('选择的目录:', selected);
      setModsDirectory(selected);

      const result = await invoke<Mod[]>('check_mods', { directory: selected }); // 传递目录
      console.log('result: ', result);
      check_mods(result);
      setMods(result);
    }
  }

  function check_mods(mods: Mod[]) {
    mods.forEach((mod) => invoke<string>('check_mod_latest_version', { id: mod.id }).then((version) => {
      console.log('version: ', version);
      mod.latest_version = version;
      setMods([...mods]);
    }).catch(() => {
      console.log('获取最新版本失败', mod.id);
      mod.latest_version = '最新版本获取失败';
      
    })
  );
  }

  function clearDirectory() {
    setModsDirectory('');
    setMods([]);
  }

  const columns = useMemo<TableColumnsType>(() => [
    { title: 'ID', dataIndex: 'id', key: 'id' },
    { title: '名称', dataIndex: 'name', key: 'name' },
    { title: '当前版本', dataIndex: 'version', key: 'version' },
    { title: '最新版本', dataIndex: 'latest_version', key: 'latest_version', render: (value) => <>{value ? <span>{value}</span> : <Spin />}</> },
    { title: '操作', key: 'action', render: (_, record) => <Button type="primary" disabled={!record['latest_version'] && record['latest_version'] !== record.version}>下载</Button> }
  ], []);

  return (
    <main className="container">
      <form action="">
        <input type="text" disabled value={modsDirectory} />
        <button type="button" onClick={handleSelectDirectory}>选择目录</button>
      </form>
      <div>
        <Button type="primary">开始下载</Button>
        <Button onClick={clearDirectory}>清空目录</Button>
      </div>
      <div>
        <Table
          dataSource={mods}
          columns={columns}
          style={{ width: 700, margin: 'auto' }}
          pagination={false}
          scroll={{ y: 500 }}
        />
      </div>
    </main>
  );
}

export default App;
