import time
import subprocess
import os
from watchdog.observers import Observer
from watchdog.events import FileSystemEventHandler

class Watcher:
    def __init__(self, directory_to_watch):
        self.DIRECTORY_TO_WATCH = directory_to_watch
        self.event_handler = Handler()
        self.observer = Observer()

    def run(self):
        self.observer.schedule(self.event_handler, self.DIRECTORY_TO_WATCH, recursive=True)
        self.observer.start()
        try:
            while True:
                time.sleep(1)
        except KeyboardInterrupt:
            self.observer.stop()
        self.observer.join()

class Handler(FileSystemEventHandler):
    def on_modified(self, event):
        if event.src_path.endswith(".py"):
            os.system('cls' if os.name == 'nt' else 'clear')  # Clear the terminal
            # print(f"File modified: {event.src_path}")
            subprocess.run(["python", "main.py"])

if __name__ == '__main__':
    path_to_watch = "."  # Watch the current directory
    watcher = Watcher(path_to_watch)
    watcher.run()
