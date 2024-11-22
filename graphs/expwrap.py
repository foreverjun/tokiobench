def trylog(f):
    try:
        f()
    except Exception as e:
        print(f"Failed to upload to ftp: {str(e)}")
