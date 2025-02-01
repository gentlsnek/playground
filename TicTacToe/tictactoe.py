import random
import time

def print_board(board):

   print(f"""
   {board[0]} | {board[1]} | {board[2]}
  ---+---+---
   {board[3]} | {board[4]} | {board[5]}
  ---+---+---
   {board[6]} | {board[7]} | {board[8]}
    """)

def check_win(board, player):
    win_combos = [
         [0, 1, 2], [3, 4, 5], [6, 7, 8],
         [0, 3, 6], [1, 4, 7], [2, 5, 8],
         [0, 4, 8], [2, 4, 6]
    ]
    
    for combo in win_combos:
         if all([board[i] == player for i in combo]):
              return True
    
    return False

def check_tie(board):
    return all([cell in ['X','O'] for cell in board])


def check_tile(tile):
    if tile == ' ':
       return False
    else:
        return True
    

def bot(board, turn):
    available_moves = [i for i in range(9) if board[i] == ' ']
    if not available_moves:
        return None
    
    random.shuffle(available_moves) 
    return random.choice(available_moves)
     

def tictactoe(rounds):
    x_wins = 0
    o_wins = 0
    draws = 0

    board = [' ' for x in range(9)]
    turn = "X"

    print("Welcome to Tic Tac Toe!")
    

    for x in range(rounds):
        random.seed(time.time() + x)  # Change seed each round
        board = [' ' for _ in range(9)]
        turn = "X"
        while True:
            move = bot(board,turn)

            if move is None:
                draws += 1
                break
         
            board[move] = turn
            #print_board(board)

            if check_win(board, turn):
                if turn == "X":
                    x_wins += 1
                else:
                    o_wins += 1
                break              

            if check_tie(board):
                    draws += 1
                    break    
            

            turn = 'O' if turn == 'X' else 'X'
        print(f"\n--- Round {x  + 1} ---")
        print_board(board)


    total_games = x_wins + o_wins + draws
    x_win_percentage = (x_wins / total_games) * 100
    o_win_percentage = (o_wins / total_games) * 100
    draw_percentage = (draws / total_games) * 100

    print("\n--- Results ---")
    print(f"Total Games: {total_games}")
    print(f"X Wins: {x_wins} ({x_win_percentage:.2f}%)")
    print(f"O Wins: {o_wins} ({o_win_percentage:.2f}%)")
    print(f"Draws: {draws} ({draw_percentage:.2f}%)")


        
            
if __name__ == "__main__":
    rounds = int(input("Enter the number of rounds: "))
    tictactoe(rounds)