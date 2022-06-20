-- Add migration script here
ALTER TABLE Todos ADD user_id SERIAL;
ALTER TABLE Todos ADD CONSTRAINT FK_UserTodo FOREIGN KEY (user_id) REFERENCES Users(id);